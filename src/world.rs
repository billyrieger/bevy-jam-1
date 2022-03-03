use crate::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sync_physics_coords).add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(sync_transforms)
                .with_system(sync_physics_coords),
        );
    }
}

fn sync_physics_coords(mut query: Query<(&mut WorldPosition, &RigidBodyPositionComponent)>) {
    for (mut coords, body_position) in query.iter_mut() {
        coords.0 = body_position.position.translation.into();
    }
}

fn sync_transforms(
    mut query: Query<(&mut Transform, &WorldPosition), With<SyncWorldPosition>>,
) {
    for (mut transform, world_coords) in query.iter_mut() {
        let depth_scale = 1.0 - 0.0055 * world_coords.0.y;
        let new_x = world_coords.0.x * WORLD_SCALE * depth_scale;
        let new_y = world_coords.0.y * WORLD_SCALE * depth_scale;
        transform.translation =
            Vec2::new(new_x, new_y).extend(3.0);
        transform.scale = Vec3::splat(PX_SCALE*depth_scale);
    }
}
