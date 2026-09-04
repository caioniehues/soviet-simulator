use crate::authent::{Client, ClientGameState};
use crate::connection_client::ConnectionClient;
use crate::connections::Connections;
use crate::packets::{ClientReliablePacket, ServerReliablePacket, WorldDataFragment};
use crate::{decode, encode, AuthentID, Frame, MAX_WORLDSEND_PACKET_SIZE};
use common::FastMap;
use serde::de::DeserializeOwned;

/// Upper bound on a received world image. Every field of an inbound fragment is
/// peer-controlled, so the accumulator must never grow without limit: a peer that
/// exceeds this bound fails its transfer (`Errored`, which the client poll loop
/// turns into a disconnect) instead of being accommodated.
pub(crate) const MAX_WORLD_RECEIVE_SIZE: usize = 64 * 1024 * 1024;

#[derive(Eq, PartialEq)]
enum WorldSendStatus {
    ReadyToSend,
    WaitingForFinalAck,
    Over,
}

struct WorldSendState {
    data: Vec<u8>,
    sent: usize,
    status: WorldSendStatus,
    frame: Frame,
}

#[derive(Default)]
pub(crate) struct WorldSend {
    send_state: FastMap<AuthentID, WorldSendState>,
}

impl WorldSend {
    pub fn begin_send(&mut self, c: &Client, data: Vec<u8>, frame: Frame) {
        self.send_state.insert(
            c.id,
            WorldSendState {
                data,
                sent: 0,
                status: WorldSendStatus::ReadyToSend,
                frame,
            },
        );
    }

    pub fn ack(&mut self, c: &Client) {
        if let Some(state) = self.send_state.get_mut(&c.id) {
            if state.status == WorldSendStatus::WaitingForFinalAck {
                state.status = WorldSendStatus::Over
            }
        } else {
            log::warn!(
                "ack ing a non existing world send. can be caused by udp duplication. is ok.",
            );
        }
    }

    pub fn update(&mut self, c: &mut Client, net: &Connections) {
        if let Some(state) = self.send_state.get_mut(&c.id) {
            if state.status == WorldSendStatus::Over {
                self.send_state.remove(&c.id);
                c.state = ClientGameState::CatchingUp;
                return;
            }
            if state.status != WorldSendStatus::ReadyToSend {
                return;
            }

            let to_send = MAX_WORLDSEND_PACKET_SIZE.min(state.data.len() - state.sent);
            let is_over = (to_send < MAX_WORLDSEND_PACKET_SIZE).then_some(state.frame);

            net.send_tcp(
                c.tcp_addr,
                encode(&ServerReliablePacket::WorldSend(WorldDataFragment {
                    is_over,
                    data_size: state.data.len(),
                    data: Vec::from(&state.data[state.sent..state.sent + to_send]),
                })),
            );

            if is_over.is_some() {
                log::info!("sending final world fragment to {}", c.name);
                state.status = WorldSendStatus::WaitingForFinalAck;
            } else {
                log::info!("sending world fragment to {}", c.name);
            }

            state.sent += to_send;
        } else {
            log::error!("updating a non existing world send");
        }
    }

    pub fn disconnected(&mut self, id: AuthentID) {
        self.send_state.remove(&id);
    }
}

#[derive(Debug)]
pub(crate) enum WorldReceive<W> {
    Downloading {
        datasize: usize,
        data_so_far: Vec<u8>,
    },
    Finished {
        frame: Frame,
        world: W,
    },
    Errored,
}

impl<W> WorldReceive<W> {
    pub fn progress(&self) -> Option<(usize, usize)> {
        match self {
            WorldReceive::Downloading {
                datasize,
                data_so_far,
            } => Some((data_so_far.len(), *datasize)),
            _ => None,
        }
    }
}

impl<W> Default for WorldReceive<W> {
    fn default() -> Self {
        Self::Downloading {
            datasize: 0,
            data_so_far: vec![],
        }
    }
}

impl<W: DeserializeOwned> WorldReceive<W> {
    pub fn handle(&mut self, fragment: WorldDataFragment, net: &ConnectionClient) {
        if let WorldReceive::Downloading {
            ref mut datasize,
            ref mut data_so_far,
        } = self
        {
            // Every field here is peer-controlled: bound the declared total first,
            // so a lying fragment cannot trick the reservation below into
            // grabbing gigabytes.
            if fragment.data_size > MAX_WORLD_RECEIVE_SIZE {
                log::warn!("world fragment declares oversized world, failing transfer");
                *self = WorldReceive::Errored;
                return;
            }
            if data_so_far.capacity() == 0 {
                *datasize = fragment.data_size;
                data_so_far.reserve(fragment.data_size)
            } else if fragment.data_size != *datasize {
                // The first fragment fixes the total; moving the goalposts
                // mid-transfer is hostile.
                log::warn!("world fragment changed declared size mid-transfer, failing");
                *self = WorldReceive::Errored;
                return;
            }
            // Bound the accumulator itself: a drip-feed of small fragments must
            // never grow the buffer past the declared total (itself capped above).
            if data_so_far.len().saturating_add(fragment.data.len()) > *datasize {
                log::warn!("world fragment overruns declared size, failing transfer");
                *self = WorldReceive::Errored;
                return;
            }
            data_so_far.extend(fragment.data);
            if let Some(frame) = fragment.is_over {
                log::info!("received last fragment at {:?}", frame);
                net.send_tcp(encode(&ClientReliablePacket::WorldAck));

                let d = decode(data_so_far);

                if let Some(w) = d {
                    *self = WorldReceive::Finished { frame, world: w }
                } else {
                    *self = WorldReceive::Errored;
                }
            }
        } else {
            log::warn!(
                "received fragment but was not downloading (errored: {:?})",
                matches!(self, WorldReceive::Errored)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    fn loopback_net() -> (TcpListener, ConnectionClient) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let addr = listener.local_addr().expect("loopback addr");
        let net = ConnectionClient::new(addr).expect("loopback connect");
        (listener, net)
    }

    #[test]
    fn hostile_world_declared_size_over_bound_is_refused() {
        let (_listener, net) = loopback_net();
        let mut recv: WorldReceive<Vec<u8>> = WorldReceive::default();
        // A single lying fragment declares a world past the bound. This must fail
        // before any reservation, never attempt to reserve the declared size.
        recv.handle(
            WorldDataFragment {
                is_over: None,
                data_size: MAX_WORLD_RECEIVE_SIZE + 1,
                data: vec![0u8; 8],
            },
            &net,
        );
        assert!(matches!(recv, WorldReceive::Errored));
    }

    #[test]
    fn hostile_world_drip_feed_over_declared_size_is_refused() {
        let (_listener, net) = loopback_net();
        let mut recv: WorldReceive<Vec<u8>> = WorldReceive::default();
        recv.handle(
            WorldDataFragment {
                is_over: None,
                data_size: 8,
                data: vec![0u8; 8],
            },
            &net,
        );
        assert!(matches!(recv, WorldReceive::Downloading { .. }));
        // A second fragment would push the accumulator past the declared total.
        recv.handle(
            WorldDataFragment {
                is_over: None,
                data_size: 8,
                data: vec![0u8; 8],
            },
            &net,
        );
        assert!(matches!(recv, WorldReceive::Errored));
    }
}
