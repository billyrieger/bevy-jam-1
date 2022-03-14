use crate::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraView::default());
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(polyline_path_system)
                .with_system(polygon_path_system)
                .with_system(sync_physics_coords),
        )
        .add_system_to_stage(CoreStage::PostUpdate, world_position_sync_system);
    }
}

// Camera points in the negative z direction with the image plane at z=0
#[derive(Debug)]
pub struct CameraView {
    pub position: Vec3,
}

impl Default for CameraView {
    fn default() -> Self {
        Self {
            position: Vec3::new(0., 5., 24.),
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

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WorldPosition(pub Vec3);

#[derive(Component)]
pub struct WorldPositionSync;

fn world_position_sync_system(
    camera_view: Res<CameraView>,
    mut query: Query<(&mut Transform, &WorldPosition), With<WorldPositionSync>>,
) {
    for (mut transform, world_coords) in query.iter_mut() {
        let depth_scale = camera_view.depth_scale(world_coords.0);
        transform.translation = camera_view.to_screen(world_coords.0).extend(depth_scale);
        transform.scale = Vec3::splat(depth_scale);
    }
}

#[derive(Component, Default)]
pub struct WorldPolyline {
    pub segments: Vec<(Vec3, Vec3)>,
}

fn polyline_path_system(
    view: Res<CameraView>,
    mut q_line: Query<(&mut Path, &WorldPolyline, &WorldPosition)>,
) {
    for (mut path, polyline, center) in q_line.iter_mut() {
        let mut builder = PathBuilder::new();
        for &(p0, p1) in &polyline.segments {
            builder.move_to(view.to_screen(center.0 + p0));
            builder.line_to(view.to_screen(center.0 + p1));
        }
        *path = builder.build();
    }
}

#[derive(Component, Default)]
pub struct WorldPolygon {
    pub corners: Vec<Vec3>,
}

fn polygon_path_system(
    view: Res<CameraView>,
    mut q_polygon: Query<(&mut Path, &WorldPolygon, &WorldPosition)>,
) {
    for (mut path, polygon, center) in q_polygon.iter_mut() {
        let points = polygon
            .corners
            .iter()
            .map(|&p| view.to_screen(center.0 + p))
            .collect();
        *path = ShapePath::new()
            .add(&shapes::Polygon {
                points,
                closed: true,
            })
            .build();
    }
}

#[derive(Component, Default)]
pub struct WorldSprite {
    pub base: Vec2,
}

fn sync_physics_coords(mut query: Query<(&mut WorldPosition, &RigidBodyPositionComponent)>) {
    for (mut coords, body_position) in query.iter_mut() {
        coords.0 = body_position.position.translation.into();
    }
}
