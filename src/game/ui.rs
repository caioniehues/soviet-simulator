//! The state-document UI vocabulary (R0.3, #113) and the severity model that
//! drives it (R0.4, #114).
//!
//! Before this module the HUD was a debug readout: four absolutely-positioned
//! panels, one text colour, ASCII progress bars, and a starving warehouse
//! rendered in exactly the same size and weight as an idle-truck count. That
//! last one is a functional defect, not a matter of taste — a player cannot
//! act on a warning they cannot see.
//!
//! So presentation gets two things it did not have:
//!
//! - **A vocabulary.** [`Theme`] holds the fonts and the palette-derived
//!   surface colours; [`panel`], [`header`], [`body`] and [`meter`] are the
//!   only sanctioned ways to draw HUD chrome. Every panel in the game is built
//!   from them, including the fullscreen Plan ledger, so the whole interface
//!   reads as one document.
//! - **A severity.** [`Severity`] is a value presentation code *reads*, rather
//!   than a colour each call site picks. A critical line is signal-fail red,
//!   bold, and prefixed with a mark; nothing else in the HUD is allowed to be.

use bevy::prelude::*;

use super::palette::Role;

/// How much of the player's attention a line is entitled to.
///
/// The ordering is meaningful: `max` of two severities is the severity of the
/// pair, which is how a panel header summarises its rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Severity {
    /// The normal state of the world. Body text, muted.
    #[default]
    Routine,
    /// Worth a look this period — a queue growing, a stock drifting low.
    Attention,
    /// The player must act, and the plan is already losing: a starving
    /// building, a blackout, a refusal, a blocked site.
    Critical,
}

impl Severity {
    pub fn color(self) -> Color {
        match self {
            Severity::Routine => Theme::INK,
            Severity::Attention => Role::SignalAttention.color(),
            Severity::Critical => CRITICAL_INK,
        }
    }

    /// A glyph carries the severity where colour cannot — on a still frame, in
    /// a screenshot, for a player who reads shape before hue.
    pub fn mark(self) -> &'static str {
        match self {
            Severity::Routine => "",
            Severity::Attention => "! ",
            Severity::Critical => "!! ",
        }
    }

    /// Critical lines are set in the bold face; the rest are regular.
    pub fn is_bold(self) -> bool {
        self == Severity::Critical
    }
}

/// Signal-fail red is a *surface* colour: at HUD sizes, `#a12a1c` on a
/// near-black panel is too dark to read. The alarm keeps the palette's hue and
/// buys back the value it needs to be legible as text.
const CRITICAL_INK: Color = Color::srgb(0.93, 0.35, 0.28);

/// Fonts and surfaces, resolved once at startup so no call site loads its own.
#[derive(Resource, Clone)]
pub struct Theme {
    pub regular: Handle<Font>,
    pub bold: Handle<Font>,
}

impl Theme {
    /// The document's own colours. Panels are near-black so the world reads
    /// through them; ink is warm off-white, never pure (the art doc's albedo
    /// rule applied to the interface).
    pub const PANEL: Color = Color::srgba(0.055, 0.060, 0.070, 0.88);
    pub const PANEL_SUNK: Color = Color::srgba(0.020, 0.024, 0.030, 0.92);
    pub const RULE: Color = Color::srgba(0.62, 0.58, 0.50, 0.28);
    pub const INK: Color = Color::srgb(0.90, 0.88, 0.81);
    pub const INK_DIM: Color = Color::srgb(0.62, 0.61, 0.57);
    /// The rust accent that marks every panel's left edge.
    pub const ACCENT: Color = Color::srgb(0.63, 0.35, 0.20);

    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            regular: asset_server.load("fonts/FiraSans-Regular.ttf"),
            bold: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }

    pub fn font(&self, size: f32, bold: bool) -> TextFont {
        TextFont {
            font: if bold {
                self.bold.clone().into()
            } else {
                self.regular.clone().into()
            },
            font_size: bevy::text::FontSize::Px(size),
            ..default()
        }
    }
}

/// The typographic scale. Three sizes, no more — a document with five sizes
/// has no hierarchy, it has noise.
pub const SIZE_HEADER: f32 = 12.0;
pub const SIZE_BODY: f32 = 15.0;
pub const SIZE_LEAD: f32 = 19.0;

/// Panel chrome: dark ground, rust accent down the left edge, and the padding
/// grid every panel shares.
///
/// Returns the concrete tuple rather than `impl Bundle` so a caller can spread
/// the `Node` (`..panel().0`) when it needs its own position — several panels
/// do, and hiding the type behind `impl` would force each of them to
/// hand-roll the chrome it is here to standardise.
pub fn panel() -> (Node, BackgroundColor, BorderColor) {
    (
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::axes(Val::Px(13.0), Val::Px(10.0)),
            border: UiRect::left(Val::Px(3.0)),
            border_radius: BorderRadius::px(1.0, 5.0, 5.0, 1.0),
            row_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(Theme::PANEL),
        BorderColor::all(Theme::ACCENT),
    )
}

/// A section header. Bevy's text stack has no letter-spacing, so the spacing
/// is baked into the string — the classic trick, and it survives any font.
pub fn header(theme: &Theme, text: &str) -> impl Bundle {
    (
        Text::new(letterspace(text)),
        theme.font(SIZE_HEADER, true),
        TextColor(Theme::INK_DIM),
    )
}

/// `PLAN` → `P L A N`. Uppercased first, so callers can write prose.
pub fn letterspace(text: &str) -> String {
    text.to_uppercase()
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\u{2009}")
}

/// Body text at a severity. The whole point of the severity model is that this
/// is the only place a HUD line decides how loud it is.
pub fn body(theme: &Theme, text: &str, severity: Severity) -> impl Bundle {
    (
        Text::new(text.to_string()),
        theme.font(SIZE_BODY, severity.is_bold()),
        TextColor(severity.color()),
    )
}

/// A hairline rule between groups inside one panel.
pub fn rule() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: UiRect::vertical(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(Theme::RULE),
    )
}

// ------------------------------------------------------------------ readouts

/// One line of a live readout, carrying the attention it is entitled to.
///
/// This is the type that fixes the R0.4 defect. A readout system now emits
/// `Line`s instead of one flat `String`, so a starving warehouse can be loud
/// and an idle-truck count quiet *within the same panel*, which a single
/// `Text` could never express.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub text: String,
    pub severity: Severity,
    /// Secondary text — captions, units, sub-items. Dim is a *rank* below
    /// routine, not a fourth severity: it never competes for attention, so it
    /// has no mark and no bold.
    pub dim: bool,
}

impl Line {
    fn at(severity: Severity, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            severity,
            dim: false,
        }
    }

    pub fn routine(text: impl Into<String>) -> Self {
        Self::at(Severity::Routine, text)
    }

    pub fn attention(text: impl Into<String>) -> Self {
        Self::at(Severity::Attention, text)
    }

    pub fn critical(text: impl Into<String>) -> Self {
        Self::at(Severity::Critical, text)
    }

    pub fn dim(text: impl Into<String>) -> Self {
        Self {
            dim: true,
            ..Self::at(Severity::Routine, text)
        }
    }
}

/// A panel's live text, drawn as a fixed pool of spans.
///
/// The pool is fixed on purpose: rebuilding child entities every frame is how
/// UI code ends up allocating in the render loop. Spans past the current line
/// count are blanked, not despawned.
#[derive(Component)]
pub struct Readout {
    spans: Vec<Entity>,
}

/// Spawn a readout with room for `capacity` lines, returning nothing — the
/// [`Readout`] component on the spawned entity is the handle.
pub fn spawn_readout(parent: &mut ChildSpawnerCommands, theme: &Theme, capacity: usize) {
    let mut spans = Vec::with_capacity(capacity);
    parent
        .spawn((
            Text::new(""),
            theme.font(SIZE_BODY, false),
            TextColor(Theme::INK),
        ))
        .with_children(|root| {
            for _ in 0..capacity {
                spans.push(
                    root.spawn((
                        TextSpan::new(""),
                        theme.font(SIZE_BODY, false),
                        TextColor(Theme::INK),
                    ))
                    .id(),
                );
            }
        })
        .insert(Readout { spans });
}

/// Write `lines` into a readout's spans. Only changed spans are touched, so
/// change detection keeps a static panel free.
pub fn write_readout(
    readout: &Readout,
    theme: &Theme,
    lines: &[Line],
    spans: &mut Query<(&mut TextSpan, &mut TextColor, &mut TextFont)>,
) {
    // The span pool is fixed, so overflow silently drops the tail — where
    // producers tend to put their loudest line (the dispatch panel pushed
    // STARVING last, below one dim line per depot). When lines overflow,
    // keep every non-routine line and let the dim tail take the cut.
    let mut kept: Vec<&Line>;
    let lines: &[&Line] = if lines.len() > readout.spans.len() {
        kept = lines
            .iter()
            .filter(|l| l.severity > Severity::Routine)
            .collect();
        let spare = readout.spans.len().saturating_sub(kept.len());
        for line in lines
            .iter()
            .filter(|l| l.severity <= Severity::Routine)
            .take(spare)
        {
            kept.push(line);
        }
        &kept
    } else {
        kept = lines.iter().collect();
        &kept
    };
    for (i, &entity) in readout.spans.iter().enumerate() {
        let Ok((mut span, mut color, mut font)) = spans.get_mut(entity) else {
            continue;
        };
        let (text, severity, dim) = match lines.get(i) {
            Some(line) => (
                format!("{}{}\n", line.severity.mark(), line.text),
                line.severity,
                line.dim,
            ),
            None => (String::new(), Severity::Routine, false),
        };
        if span.0 != text {
            span.0 = text;
        }
        let want_color = if dim {
            Theme::INK_DIM
        } else {
            severity.color()
        };
        if color.0 != want_color {
            color.0 = want_color;
        }
        let want_bold = severity.is_bold();
        let want_font = theme.font(SIZE_BODY, want_bold);
        if font.font != want_font.font {
            *font = want_font;
        }
    }
}

// -------------------------------------------------------------------- meters

/// A drawn progress bar. Replaces the ASCII `[####····]` bars, which were a
/// stand-in for exactly this and read as a terminal, not a document.
///
/// The value and severity live on the component; [`sync_meters`] pushes them
/// into the child fill node, so a readout system only has to write a number.
#[derive(Component, Clone, Copy)]
pub struct Meter {
    /// 0..=1; values above 1 clamp, because a quota can be overfulfilled and
    /// the bar should read "full", not overflow its track.
    pub value: f32,
    pub severity: Severity,
}

impl Default for Meter {
    fn default() -> Self {
        Self {
            value: 0.0,
            severity: Severity::Routine,
        }
    }
}

/// The child node whose width is the fill.
#[derive(Component)]
pub struct MeterFill;

/// A meter's fill colour: signal green-gold when it is doing well, the
/// severity's colour when it is not.
fn fill_color(meter: &Meter) -> Color {
    match meter.severity {
        Severity::Routine if meter.value >= 0.999 => Role::SignalOk.color(),
        Severity::Routine => Color::srgb(0.55, 0.62, 0.45),
        other => other.color(),
    }
}

/// Spawn a labelled meter row: caption line above, track below. `marker` is
/// put on the row node so a readout can find its own rows again — the label
/// and the track are then this row's children.
pub fn spawn_meter(
    parent: &mut ChildSpawnerCommands,
    theme: &Theme,
    label: &str,
    width: f32,
    marker: impl Bundle,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                ..default()
            },
            marker,
        ))
        .with_children(|row| {
            row.spawn((
                Text::new(label.to_string()),
                theme.font(SIZE_BODY, false),
                TextColor(Theme::INK),
                MeterLabel,
            ));
            row.spawn((
                Node {
                    width: Val::Px(width),
                    height: Val::Px(7.0),
                    border_radius: BorderRadius::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Theme::PANEL_SUNK),
                Meter::default(),
            ))
            .with_children(|track| {
                track.spawn((
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    MeterFill,
                ));
            });
        });
}

/// The caption above a meter, so a readout can rewrite it without knowing the
/// row's structure.
#[derive(Component)]
pub struct MeterLabel;

/// Push each meter's value and severity into its fill node. Runs on change
/// detection only — a HUD with twenty static meters costs nothing per frame.
pub fn sync_meters(
    meters: Query<(&Meter, &Children), Changed<Meter>>,
    mut fills: Query<(&mut Node, &mut BackgroundColor), With<MeterFill>>,
) {
    for (meter, children) in &meters {
        for child in children.iter() {
            let Ok((mut node, mut color)) = fills.get_mut(child) else {
                continue;
            };
            node.width = Val::Percent(meter.value.clamp(0.0, 1.0) * 100.0);
            color.0 = fill_color(meter);
        }
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_theme)
            .add_systems(Update, sync_meters);
    }
}

fn load_theme(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Theme::load(&asset_server));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_orders_so_a_panel_can_summarise_its_rows() {
        assert!(Severity::Critical > Severity::Attention);
        assert!(Severity::Attention > Severity::Routine);
        assert_eq!(
            [Severity::Routine, Severity::Critical, Severity::Attention]
                .into_iter()
                .max(),
            Some(Severity::Critical)
        );
    }

    #[test]
    fn critical_is_marked_as_well_as_coloured() {
        // Colour alone fails on a still frame and for colour-blind readers;
        // the mark is the redundant channel.
        assert!(!Severity::Critical.mark().is_empty());
        assert!(Severity::Routine.mark().is_empty());
        assert!(Severity::Critical.is_bold());
    }

    #[test]
    fn headers_are_letterspaced_and_uppercased() {
        assert_eq!(letterspace("plan"), "P\u{2009}L\u{2009}A\u{2009}N");
    }

    #[test]
    fn a_full_routine_meter_reads_as_fulfilled() {
        let full = Meter {
            value: 1.0,
            severity: Severity::Routine,
        };
        assert_eq!(fill_color(&full), Role::SignalOk.color());
        let starving = Meter {
            value: 0.1,
            severity: Severity::Critical,
        };
        assert_eq!(fill_color(&starving), Severity::Critical.color());
    }
}
