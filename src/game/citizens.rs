//! Commuter pawn rendering: a small capsule figure per travelling citizen,
//! eased toward the sim-authoritative position (ADR 0003). Muted overcoat
//! tones per docs/art-direction.md; identity stays in the sim.

use bevy::prelude::*;

use crate::sim::commute::CommuterPawn;

#[derive(Resource)]
struct PawnVisuals {
    mesh: Handle<Mesh>,
    coats: [Handle<StandardMaterial>; 3],
}

pub struct CitizenViewPlugin;

impl Plugin for CitizenViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_visuals)
            .add_systems(Update, (dress_new_pawns, sync_pawn_transforms));
    }
}

fn setup_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let coat = |color: Color| StandardMaterial {
        base_color: color,
        perceptual_roughness: 1.0,
        ..default()
    };
    commands.insert_resource(PawnVisuals {
        mesh: meshes.add(Capsule3d::new(0.35, 1.1)),
        coats: [
            materials.add(coat(Color::srgb(0.30, 0.28, 0.25))),
            materials.add(coat(Color::srgb(0.35, 0.30, 0.22))),
            materials.add(coat(Color::srgb(0.25, 0.27, 0.30))),
        ],
    });
}

fn dress_new_pawns(
    mut commands: Commands,
    added: Query<(Entity, &CommuterPawn), Added<CommuterPawn>>,
    visuals: Res<PawnVisuals>,
) {
    for (entity, pawn) in &added {
        let coat = visuals.coats[pawn.citizen.to_bits() as usize % visuals.coats.len()].clone();
        commands.entity(entity).insert((
            Mesh3d(visuals.mesh.clone()),
            MeshMaterial3d(coat),
            Transform::from_translation(pawn.pos + Vec3::Y * 0.9),
            Name::new("Commuter"),
        ));
    }
}

fn sync_pawn_transforms(
    time: Res<Time>,
    mut pawns: Query<(&CommuterPawn, &mut Transform, &mut Visibility)>,
) {
    let ease = (time.delta_secs() * 14.0).min(1.0);
    for (pawn, mut transform, mut visibility) in &mut pawns {
        let target = pawn.pos + Vec3::Y * 0.9;
        transform.translation = transform.translation.lerp(target, ease);
        // A rider is inside the bus, not standing on its roof (B5.5).
        let wanted = if matches!(pawn.phase, crate::sim::commute::CommutePhase::Ride { .. }) {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        if *visibility != wanted {
            *visibility = wanted;
        }
    }
}
