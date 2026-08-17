//! Notifications and the event log (R0.4, #114).
//!
//! The charter's rule is narrow and load-bearing: **action-needed events
//! only**. A toast that fires for everything is a toast the player learns to
//! ignore, and then the one that mattered goes unread too. So the board takes
//! a [`Notice`] only for things the player can *do* something about — a
//! refused spend, a material shortfall, a blocked site, a period result — and
//! everything else stays in the panels where it belongs.
//!
//! Two surfaces, one source:
//!
//! - **Toasts** slide in at the top centre, hold, then fade. They are the
//!   interruption.
//! - **The event log** keeps the last [`LOG_CAPACITY`] notices in a panel
//!   toggled with `L`, so a player who looked away can catch up. It is the
//!   record.
//!
//! Repeats are folded rather than stacked: a warehouse starving for six
//! seconds is one toast with a counter, not six toasts.

use bevy::prelude::*;

use super::ui::{Line, Readout, Severity, Theme, panel, spawn_readout, write_readout};

/// How long a toast holds at full opacity before it begins to fade.
const TOAST_HOLD: f32 = 4.5;
const TOAST_FADE: f32 = 1.2;
/// A repeat within this window folds into the existing toast.
const FOLD_WINDOW: f32 = 12.0;
/// Toasts on screen at once; older ones are retired early to make room.
const TOAST_LIMIT: usize = 4;
pub const LOG_CAPACITY: usize = 14;

/// Something the player needs to act on.
#[derive(Clone, Debug)]
pub struct Notice {
    pub text: String,
    pub severity: Severity,
}

impl Notice {
    pub fn critical(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            severity: Severity::Critical,
        }
    }

    pub fn attention(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            severity: Severity::Attention,
        }
    }

    pub fn routine(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            severity: Severity::Routine,
        }
    }
}

/// The single door notices come through. Any system — sim-adjacent or
/// presentation — pushes here; only this module decides how it is shown.
#[derive(Resource, Default)]
pub struct NoticeBoard {
    pending: Vec<Notice>,
    log: Vec<LoggedNotice>,
}

#[derive(Clone)]
struct LoggedNotice {
    notice: Notice,
    repeats: u32,
}

impl NoticeBoard {
    pub fn push(&mut self, notice: Notice) {
        self.pending.push(notice);
    }

    pub fn log(&self) -> impl Iterator<Item = (&str, Severity, u32)> {
        self.log
            .iter()
            .rev()
            .map(|l| (l.notice.text.as_str(), l.notice.severity, l.repeats))
    }
}

#[derive(Component)]
struct Toast {
    text: String,
    age: f32,
    repeats: u32,
    /// The severity's colour, kept so the fade can rebuild the border alpha
    /// from the original rather than compounding its own output.
    accent: Color,
}

#[derive(Component)]
struct ToastLane;

#[derive(Component)]
struct EventLogPanel;

pub struct NotifyPlugin;

impl Plugin for NotifyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoticeBoard>()
            .add_systems(Startup, spawn_surfaces)
            .add_systems(
                Update,
                (
                    drain_board,
                    age_toasts,
                    toggle_event_log,
                    update_event_log.after(drain_board),
                )
                    .chain(),
            );
    }
}

fn spawn_surfaces(mut commands: Commands, theme: Res<Theme>) {
    // Toast lane: top centre, above the world, below nothing. Interruptions
    // belong where the eye already is when something changes.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(14.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-210.0)),
            width: Val::Px(420.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(6.0),
            ..default()
        },
        ToastLane,
        Name::new("HudToastLane"),
    ));

    // The event log: bottom right, hidden until `L`.
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(12.0),
                bottom: Val::Px(56.0),
                width: Val::Px(420.0),
                display: Display::None,
                ..panel().0
            },
            panel().1,
            panel().2,
            EventLogPanel,
            Name::new("HudEventLog"),
        ))
        .with_children(|parent| {
            parent.spawn(super::ui::header(&theme, "event log"));
            spawn_readout(parent, &theme, LOG_CAPACITY);
        });
}

/// Turn pending notices into toasts and log entries, folding repeats.
fn drain_board(
    mut commands: Commands,
    mut board: ResMut<NoticeBoard>,
    theme: Res<Theme>,
    lane: Query<Entity, With<ToastLane>>,
    mut toasts: Query<(Entity, &mut Toast)>,
) {
    if board.pending.is_empty() {
        return;
    }
    let Ok(lane) = lane.single() else { return };
    let pending: Vec<Notice> = board.pending.drain(..).collect();

    for notice in pending {
        // Fold into a live toast of the same text rather than stacking.
        if let Some((_, mut toast)) = toasts
            .iter_mut()
            .find(|(_, t)| t.text == notice.text && t.age < FOLD_WINDOW)
        {
            toast.repeats += 1;
            toast.age = 0.0;
            if let Some(entry) = board.log.iter_mut().find(|l| l.notice.text == notice.text) {
                entry.repeats += 1;
            }
            continue;
        }

        // Oldest goes first when the lane is full — the newest event is the
        // one the player has not yet seen.
        let live: Vec<(Entity, f32)> = toasts.iter().map(|(e, t)| (e, t.age)).collect();
        if live.len() >= TOAST_LIMIT
            && let Some(&(oldest, _)) = live
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        {
            commands.entity(oldest).despawn();
        }

        commands.entity(lane).with_children(|lane| {
            lane.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(9.0)),
                    border: UiRect::left(Val::Px(4.0)),
                    border_radius: BorderRadius::px(1.0, 4.0, 4.0, 1.0),
                    ..default()
                },
                BackgroundColor(Theme::PANEL),
                BorderColor::all(notice.severity.color()),
                Toast {
                    text: notice.text.clone(),
                    age: 0.0,
                    repeats: 1,
                    accent: notice.severity.color(),
                },
                children![(
                    Text::new(format!("{}{}", notice.severity.mark(), notice.text)),
                    theme.font(super::ui::SIZE_BODY, notice.severity.is_bold()),
                    TextColor(notice.severity.color()),
                )],
            ));
        });

        board.log.push(LoggedNotice { notice, repeats: 1 });
        if board.log.len() > LOG_CAPACITY {
            board.log.remove(0);
        }
    }
}

/// Hold, then fade, then retire. The fade is on the panel and its text alike,
/// so a toast never leaves a ghost border behind.
fn age_toasts(
    mut commands: Commands,
    time: Res<Time>,
    mut toasts: Query<(
        Entity,
        &mut Toast,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
) {
    for (entity, mut toast, mut bg, mut border, children) in &mut toasts {
        toast.age += time.delta_secs();
        if toast.age > TOAST_HOLD + TOAST_FADE {
            commands.entity(entity).despawn();
            continue;
        }
        let alpha = ((TOAST_HOLD + TOAST_FADE - toast.age) / TOAST_FADE).clamp(0.0, 1.0);
        bg.0.set_alpha(alpha * Theme::PANEL.alpha());
        let mut accent = toast.accent;
        accent.set_alpha(alpha);
        border.set_all(accent);
        for child in children.iter() {
            let Ok((mut text, mut color)) = texts.get_mut(child) else {
                continue;
            };
            color.0.set_alpha(alpha);
            if toast.repeats > 1 {
                let want = format!("{}  (\u{d7}{})", toast.text, toast.repeats);
                if text.0 != want {
                    text.0 = want;
                }
            }
        }
    }
}

fn toggle_event_log(
    keys: Res<ButtonInput<KeyCode>>,
    mut panel: Query<&mut Node, With<EventLogPanel>>,
) {
    if !keys.just_pressed(KeyCode::KeyL) {
        return;
    }
    for mut node in &mut panel {
        node.display = match node.display {
            Display::None => Display::Flex,
            _ => Display::None,
        };
    }
}

fn update_event_log(
    board: Res<NoticeBoard>,
    theme: Res<Theme>,
    panel: Query<(&Node, &Children), With<EventLogPanel>>,
    readouts: Query<&Readout>,
    mut spans: Query<(&mut TextSpan, &mut TextColor, &mut TextFont)>,
) {
    let Ok((node, children)) = panel.single() else {
        return;
    };
    // Skip the string build while hidden — the same discipline the Plan
    // ledger already keeps.
    if node.display == Display::None {
        return;
    }
    let Some(readout) = children.iter().find_map(|c| readouts.get(c).ok()) else {
        return;
    };
    let lines: Vec<Line> = board
        .log()
        .map(|(text, severity, repeats)| {
            let text = if repeats > 1 {
                format!("{text}  (\u{d7}{repeats})")
            } else {
                text.to_string()
            };
            Line {
                text,
                severity,
                dim: false,
            }
        })
        .collect();
    write_readout(readout, &theme, &lines, &mut spans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_log_keeps_the_newest_and_reads_newest_first() {
        let mut board = NoticeBoard::default();
        for i in 0..LOG_CAPACITY + 3 {
            board.log.push(LoggedNotice {
                notice: Notice::routine(format!("event {i}")),
                repeats: 1,
            });
            if board.log.len() > LOG_CAPACITY {
                board.log.remove(0);
            }
        }
        let texts: Vec<&str> = board.log().map(|(t, _, _)| t).collect();
        assert_eq!(texts.len(), LOG_CAPACITY);
        assert_eq!(texts[0], format!("event {}", LOG_CAPACITY + 2));
        assert!(!texts.contains(&"event 0"));
    }

    #[test]
    fn pushing_queues_without_showing() {
        // The board is a door, not a renderer: pushing must not require any
        // UI to exist, so sim-adjacent code can notify freely.
        let mut board = NoticeBoard::default();
        board.push(Notice::critical("Warehouse #3 starving"));
        assert_eq!(board.pending.len(), 1);
        assert_eq!(board.log().count(), 0);
    }
}
