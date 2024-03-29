use crate::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sync_physics_coords).add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(sync_transforms)
                .with_system(sync_shadow_position_system)
                .with_system(sync_physics_coords),
        );
    }
}

const DEPTH_SCALE: f32 = 0.0055;

fn sync_physics_coords(mut query: Query<(&mut WorldPosition, &RigidBodyPositionComponent)>) {
    for (mut coords, body_position) in query.iter_mut() {
        coords.0 = body_position.position.translation.into();
    }
}

fn sync_transforms(
    mut query: Query<
        (
            &mut Transform,
            &WorldPosition,
            &WorldSprite,
            Option<&Shadow>,
        ),
        With<SyncWorldPosition>,
    >,
) {
    for (mut transform, world_coords, world_sprite, maybe_scale) in query.iter_mut() {
        let shadow_scale = maybe_scale.map(|shadow| shadow.scale).unwrap_or(1.0);
        let depth_scale = 1.0 - DEPTH_SCALE * world_coords.0.y;
        let scaled = world_coords.0 * WORLD_SCALE * depth_scale;
        transform.translation =
            (Vec2::new(scaled.x, scaled.y + scaled.z) - world_sprite.base).extend(depth_scale);
        // transform.scale = Vec3::splat(PX_SCALE * depth_scale * world_sprite.custom_scale);
        transform.scale = Vec3::splat(PX_SCALE * depth_scale * shadow_scale);
    }
}

fn sync_shadow_position_system(
    mut shadow_query: Query<(&mut Shadow, &mut WorldPosition, &mut WorldSprite)>,
    parent_query: Query<&WorldPosition, Without<Shadow>>,
) {
    for (mut shadow, mut shadow_position, mut sprite) in shadow_query.iter_mut() {
        if let Ok(parent_position) = parent_query.get(shadow.parent) {
            *shadow_position = *parent_position;
            shadow_position.0.y += 0.01;
            shadow_position.0.z = 0.;
            shadow.scale = 1.0 - parent_position.0.z * 0.03;
        }
    }
}
