use crate::*;

const DEPTH_SCALE: f32 = 0.0055;
const CAMERA_HEIGHT_DEFAULT: f32 = 40.;
const CAMERA_DEPTH_DEFAULT: f32 = 120.;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraView::default());
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(sync_transforms)
                .with_system(sync_physics_coords)
                .with_system(move_camera_view_system)
                // .with_system(sync_shadow_position_system)
                .with_system(sync_physics_coords),
        );
    }
}

// Camera points in the negative z direction with the image plane at z=0
pub struct CameraView {
    position: Vec3,
}

impl Default for CameraView {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, CAMERA_HEIGHT_DEFAULT, CAMERA_DEPTH_DEFAULT),
        }
    }
}

impl CameraView {
    pub fn depth_scale(&self, world_pos: Vec3) -> f32 {
        self.position.z / (self.position.z - world_pos.z)
    }

    pub fn to_screen(&self, world_pos: Vec3) -> Vec2 {
        let depth_scale = self.depth_scale(world_pos);
        (self.position + (world_pos - self.position) * depth_scale).truncate() * WORLD_SCALE
    }
}

fn move_camera_view_system(input: Res<Input<KeyCode>>, mut camera_view: ResMut<CameraView>) {
    if input.just_pressed(KeyCode::W) {
        camera_view.position.z -= 10.;
    }
    if input.just_pressed(KeyCode::W) {
        camera_view.position.z += 10.;
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WorldPosition(pub Vec3);

fn sync_transforms(
    camera_view: Res<CameraView>,
    mut query: Query<(&mut Transform, &WorldPosition), With<SyncWorldPosition>>,
) {
    for (mut transform, world_coords) in query.iter_mut() {
        let depth_scale = camera_view.depth_scale(world_coords.0);
        transform.translation = camera_view.to_screen(world_coords.0).extend(depth_scale);
        transform.scale = Vec3::splat(depth_scale);
    }
}

#[derive(Component, Default)]
pub struct WorldSprite {
    pub base: Vec2,
}

#[derive(Component)]
pub struct SyncWorldPosition;

#[derive(Component)]
pub struct Shadow {
    pub parent: Entity,
    pub scale: f32,
}

fn sync_physics_coords(mut query: Query<(&mut WorldPosition, &RigidBodyPositionComponent)>) {
    for (mut coords, body_position) in query.iter_mut() {
        coords.0 = body_position.position.translation.into();
    }
}

// fn sync_shadow_position_system(
//     mut shadow_query: Query<(&mut Shadow, &mut WorldPosition, &mut WorldSprite)>,
//     parent_query: Query<&WorldPosition, Without<Shadow>>,
// ) {
//     for (shadow, mut shadow_position, mut sprite) in shadow_query.iter_mut() {
//         if let Ok(parent_position) = parent_query.get(shadow.parent) {
//             *shadow_position = *parent_position;
//             shadow_position.0.y += 0.01;
//             shadow_position.0.z = 0.;
//         }
//     }
// }
