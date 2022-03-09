use crate::prelude::*;

pub struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_court_system))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    // .with_system(draw_court_system)
                    .with_system(draw_court_outline_system)
                    .with_system(draw_court_surface_system),
            );
    }
}

const FEET_TO_METERS: f32 = 0.3048;

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
            net_to_baseline: 39. * FEET_TO_METERS,
            net_to_service_line: 21. * FEET_TO_METERS,
            center_to_sideline: 13.5 * FEET_TO_METERS,
            alley_width: 4.5 * FEET_TO_METERS,
        }
    }
}

impl CourtDimensions {
    fn court_outline(&self) -> WorldPolyline {
        let baseline = self.net_to_baseline * Vec3::Z;
        let service_line = self.net_to_service_line * Vec3::Z;
        let sideline = self.center_to_sideline * Vec3::X;
        let alley = sideline + self.alley_width * Vec3::X;
        let segments = [
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
        WorldPolyline {
            segments: segments.to_vec(),
        }
    }
}

#[derive(Component)]
struct CourtData {
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
        .insert(CourtSurface)
        .id();
    let outline_entity = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &PathBuilder::new().build().0,
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.)),
            Transform::default(),
        ))
        .insert_bundle((WorldPosition::default(), WorldPolyline::default()))
        .insert(CourtOutline)
        .id();
    commands
        .spawn()
        .insert(WorldPosition(Vec3::ZERO))
        .insert(CourtData {
            outline: outline_entity,
            surface: surface_entity,
            ground: ground_entity,
        })
        .insert(CourtDimensions::default())
        .add_child(outline_entity);
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct CourtOutline;

fn draw_court_outline_system(
    view: Res<CameraView>,
    q_court: Query<(&CourtData, &CourtDimensions)>,
    mut q_outline: Query<&mut WorldPolyline, With<CourtOutline>>,
) {
    for (court, dimensions) in q_court.iter() {
        let mut outline = q_outline
            .get_mut(court.outline)
            .expect("outline entity not found");
        *outline = dimensions.court_outline();
    }
}

#[derive(Component)]
struct CourtSurface;

fn draw_court_surface_system(
    view: Res<CameraView>,
    q_court: Query<(&CourtData, &CourtDimensions, &WorldPosition)>,
    mut q_surfaces: Query<&mut Path, With<CourtSurface>>,
) {
    for (court, dimensions, _court_center) in q_court.iter() {
        let baseline = dimensions.net_to_baseline * Vec3::Z;
        let alley = (dimensions.center_to_sideline + dimensions.alley_width) * Vec3::X;

        let mut surface_path = q_surfaces
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
    }
}
