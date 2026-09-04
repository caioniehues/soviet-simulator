use crate::authent::{Client, ClientGameState};
use crate::connections::Connections;
use crate::packets::ServerReliablePacket;
use crate::{encode, AuthentID, Frame, MergedInputs};
use common::FastMap;

struct CatchUpState {
    inputs: Vec<MergedInputs>,
    sent: usize,
    from: Frame,
    ready: bool,
}

#[derive(Default)]
pub(crate) struct CatchUp {
    frame_history: FastMap<AuthentID, CatchUpState>,
}

impl CatchUp {
    pub fn begin_remembering(&mut self, from: Frame, c: &Client) {
        let v = self.frame_history.insert(
            c.id,
            CatchUpState {
                inputs: vec![],
                sent: 0,
                from,
                ready: false,
            },
        );

        if v.is_some() {
            log::error!("client was already catching up ??")
        }
    }

    pub fn add_merged_inputs(&mut self, frame: Frame, inp: MergedInputs) {
        for v in self.frame_history.values_mut() {
            if frame.0 != v.from.0 + 1 + v.inputs.len() as u64 {
                // A desync is refused, never accommodated: pushing the wrong input
                // would silently corrupt every catching-up client.
                log::error!("wrong input for catch up, refusing !!!");
                continue;
            }
            v.inputs.push(inp.clone())
        }
    }

    pub fn ack(&mut self, c: &Client) {
        if let Some(x) = self.frame_history.get_mut(&c.id) {
            x.ready = true;
        }
    }

    pub fn update(&mut self, c: &mut Client, net: &Connections) {
        let state = match self.frame_history.get_mut(&c.id) {
            Some(x) => x,
            None => return,
        };

        if !state.ready {
            return;
        }

        state.ready = false;

        let diff = state.inputs.len() - state.sent;

        let inputs = Vec::from(&state.inputs[state.sent..]);
        state.sent += inputs.len();

        c.ack = state.from + Frame(state.sent as u64);

        if diff <= 30 {
            log::info!("{}: sending final catch up", c.name);
            net.send_tcp(
                c.tcp_addr,
                encode(&ServerReliablePacket::ReadyToPlay {
                    final_consumed_frame: c.ack,
                    final_inputs: inputs,
                }),
            );
            c.state = ClientGameState::Playing;
            self.frame_history.remove(&c.id);
            return;
        }

        let pack = ServerReliablePacket::CatchUp { inputs };

        net.send_tcp(c.tcp_addr, encode(&pack));
    }

    pub fn disconnected(&mut self, id: AuthentID) {
        self.frame_history.remove(&id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UserID;
    use std::net::SocketAddr;

    fn test_client() -> Client {
        let addr: SocketAddr = "127.0.0.1:23019".parse().expect("test addr");
        Client {
            id: AuthentID(7),
            uid: UserID(7),
            name: "probe".to_string(),
            ack: Frame(10),
            udp_addr: addr,
            tcp_addr: addr,
            state: ClientGameState::CatchingUp,
        }
    }

    #[test]
    fn wrong_frame_input_is_refused_not_pushed() {
        let mut catchup = CatchUp::default();
        let c = test_client();
        catchup.begin_remembering(Frame(10), &c);
        catchup.add_merged_inputs(Frame(11), vec![]);
        assert_eq!(catchup.frame_history.get(&c.id).expect("remembering").inputs.len(), 1);
        // Frame 12 skipped: hostile or desynced input, must be refused.
        catchup.add_merged_inputs(Frame(13), vec![]);
        assert_eq!(catchup.frame_history.get(&c.id).expect("remembering").inputs.len(), 1);
        // The correct next frame is still accepted afterwards.
        catchup.add_merged_inputs(Frame(12), vec![]);
        assert_eq!(catchup.frame_history.get(&c.id).expect("remembering").inputs.len(), 2);
    }
}
