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

fn sync_physics_coords(mut query: Query<(&mut WorldCoords, &RigidBodyPositionComponent)>) {
    for (mut coords, body_position) in query.iter_mut() {
        coords.0 = body_position.position.translation.into();
    }
}

fn sync_transforms(mut query: Query<(&mut Transform, &WorldCoords), With<SyncCoords>>) {
    for (mut transform, world_coords) in query.iter_mut() {
        let scale = 1.0;
        transform.translation = Vec2::new(
            world_coords.0.x * scale,
            -world_coords.0.z * scale,
        )
        .extend(3.0);
        // let calced = scale_fn(world_coords.0.z);
        // let z = world_coords.0.z;
        // println!("z: {z}, scale: {calced}");
        transform.scale = Vec3::splat(crate::PX_SCALE * scale);
    }
}

const BG_LAYER: f32 = 0.0;
const SHADOW_LAYER: f32 = 1.0;
const BALL_LAYER: f32 = 2.0;
const PLAYER_LAYER: f32 = 3.0;

// (+-222, 119):
// (+-263, -277)

struct _GameBackground {
    center: Vec3,
    bottom_left: Vec3,
    top_right: Vec3,
}
