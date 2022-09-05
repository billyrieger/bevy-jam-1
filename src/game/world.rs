use crate::{AppState};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::prelude::*;

pub const PIXELS_PER_METER: f32 = 16.;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraView>();
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(polyline_path_system)
                .with_system(polygon_path_system),
        )
        .add_system_to_stage(CoreStage::PostUpdate, position_sync_system);
    }
}

#[derive(Component, Default, Inspectable)]
pub struct Position(pub Vec3);

#[derive(Component, Default)]
pub struct BaseOffset(pub Vec2);

#[derive(Component)]
pub struct PositionSync;

#[derive(Component)]
pub struct SimplePerspective {
    vanishing_point: Vec2,
    depth: f32,
}

pub struct ViewBundle {
    perspective: SimplePerspective,
    position: Position,
}

// Camera points in the negative z direction and the scale factor at z=0 is 1.
#[derive(Debug)]
pub struct CameraView {
    pub position: Vec3,
}

impl Default for CameraView {
    fn default() -> Self {
        Self {
            position: Vec3::new(0., 27., 58.),
        }
    }
}

impl CameraView {
    pub fn depth_scale(&self, world_pos: Vec3) -> f32 {
        // Note: the scale is 1 when world_pos.z is 0.
        self.position.z / (self.position.z - world_pos.z)
    }

    pub fn to_screen(&self, world_pos: Vec3) -> Vec2 {
        let depth_scale = self.depth_scale(world_pos);
        let world_screen = self.position + (world_pos - self.position) * depth_scale;
        world_screen.truncate() * PIXELS_PER_METER
    }
}

fn position_sync_system(
    camera_view: Res<CameraView>,
    mut query: Query<(&mut Transform, &Position, Option<&BaseOffset>), With<PositionSync>>,
) {
    for (mut transform, world_coords, sync) in query.iter_mut() {
        let depth_scale = camera_view.depth_scale(world_coords.0);
        let base_offset = sync.map(|offset| offset.0).unwrap_or_default();
        transform.translation =
            (camera_view.to_screen(world_coords.0) - base_offset).extend(depth_scale);
        transform.scale = Vec3::splat(depth_scale);
    }
}

#[derive(Component, Default)]
pub struct WorldPolyline {
    pub segments: Vec<(Vec3, Vec3)>,
}

fn polyline_path_system(
    view: Res<CameraView>,
    mut q_line: Query<(&mut Path, &WorldPolyline, &Position)>,
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
    mut q_polygon: Query<(&mut Path, &WorldPolygon, &Position)>,
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
