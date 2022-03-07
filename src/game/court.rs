use super::world::{CameraView, WorldPosition};
use crate::{AppState, WORLD_SCALE};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_court_system))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(draw_court_system));
    }
}

#[derive(Component)]
struct CourtDimensions {
    net_to_baseline: f32,
    net_to_service_line: f32,
    center_to_sideline: f32,
    alley_width: f32,
}

impl Default for CourtDimensions {
    fn default() -> Self {
        Self {
            net_to_baseline: 39.,
            net_to_service_line: 21.,
            center_to_sideline: 13.5,
            alley_width: 4.5,
        }
    }
}

pub struct SpawnCourtEvent;

#[derive(Component)]
struct Court {
    dimensions: CourtDimensions,
    ground: Entity,
    outline: Entity,
    surface: Entity,
}

fn spawn_court_system(mut commands: Commands) {
    let ground_entity = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle::default(),
            DrawMode::Fill(FillMode {
                color: Color::hex("7a9b00").unwrap(),
                options: FillOptions::DEFAULT,
            }),
            Transform::default(),
        ))
        .id();
    let surface_entity = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle::default(),
            DrawMode::Fill(FillMode {
                color: Color::hex("366d00").unwrap(),
                options: FillOptions::DEFAULT,
            }),
            Transform::default(),
        ))
        .id();
    let outline_entity = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &PathBuilder::new().build().0,
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.)),
            Transform::default(),
        ))
        .insert(CourtOutline)
        .id();
    commands
        .spawn()
        .insert(WorldPosition(Vec3::ZERO))
        .insert(Court {
            outline: outline_entity,
            surface: surface_entity,
            ground: ground_entity,
            dimensions: CourtDimensions::default(),
        })
        .add_child(outline_entity);
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct CourtOutline;

#[derive(Component)]
struct CourtSurface;

fn draw_court_system(
    view: Res<CameraView>,
    q_court: Query<(&Court, &WorldPosition)>,
    mut q_paths: Query<&mut Path>,
) {
    for (court, _court_center) in q_court.iter() {
        // draw the ground
        let mut ground_path = q_paths
            .get_mut(court.ground)
            .expect("ground entity not found");
        let horizon_line = view.position.y * WORLD_SCALE;
        let rect = shapes::Rectangle {
            extents: Vec2::new(320., 500.),
            origin: RectangleOrigin::CustomCenter(Vec2::new(0., horizon_line - 250.)),
        };
        *ground_path = ShapePath::new().add(&rect).build();

        let baseline = court.dimensions.net_to_baseline * Vec3::Z;
        let service_line = court.dimensions.net_to_service_line * Vec3::Z;
        let sideline = court.dimensions.center_to_sideline * Vec3::X;
        let alley = sideline + court.dimensions.alley_width * Vec3::X;

        let mut surface_path = q_paths
            .get_mut(court.surface)
            .expect("surface entity not found");
        let surface_corners = [
            -alley + baseline,
            alley + baseline,
            alley - baseline,
            -alley - baseline,
        ]
        .map(|p| view.to_screen(p));
        *surface_path = ShapePath::new()
            .add(&shapes::Polygon {
                points: surface_corners.to_vec(),
                closed: true,
            })
            .build();

        let mut outline_path = q_paths
            .get_mut(court.outline)
            .expect("outline entity not found");

        let segment_endpoints = [
            // net line
            (-alley, alley),
            // near & far baselines
            (-alley + baseline, alley + baseline),
            (-alley - baseline, alley - baseline),
            // near & far service lines
            (-sideline + service_line, sideline + service_line),
            (sideline - baseline, sideline + baseline),
            // right & left alleys
            (alley - baseline, alley + baseline),
            (-alley - baseline, -alley + baseline),
            // right & left sidelines
            (-sideline - baseline, -sideline + baseline),
            (-sideline - service_line, sideline - service_line),
            // center service line
            (-service_line, service_line),
        ];

        let mut path = PathBuilder::new();
        for (p0, p1) in segment_endpoints {
            path.move_to(view.to_screen(p0));
            path.line_to(view.to_screen(p1));
        }
        *outline_path = path.build();
    }
}
