use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameState;

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sync_physics_coords).add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(sync_transforms)
                .with_system(sync_physics_coords),
        );
    }
}

#[derive(Component)]
pub(crate) struct WorldCoords(pub(crate) Vec3);

#[derive(Component)]
pub(crate) struct SyncCoords;

fn sync_physics_coords(
    mut query: Query<(&mut WorldCoords, &RigidBodyPositionComponent), With<SyncCoords>>,
) {
    for (mut coords, body_position) in query.iter_mut() {
        coords.0 = body_position.position.translation.into();
    }
}

fn sync_transforms(mut query: Query<(&mut Transform, &WorldCoords)>) {
    for (mut transform, world_coords) in query.iter_mut() {
        transform.translation = (Vec2::new(world_coords.0.x, -world_coords.0.z) * 10.0).extend(3.0);
        transform.scale = Vec3::splat((world_coords.0.y + 10.0) / 10.0);
    }
}
