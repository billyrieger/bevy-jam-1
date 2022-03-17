use crate::prelude::*;

use super::world::WorldPolygon;

pub struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_court_system))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_court_outline_system)
                    .with_system(update_court_surface_system),
            );
    }
}

const FEET_TO_METERS: f32 = 0.3048;

#[derive(Component)]
struct CourtDimensions {
    net_to_baseline: f32,
    net_to_service_line: f32,
    center_to_sideline: f32,
    center_to_alley: f32,
}

impl Default for CourtDimensions {
    fn default() -> Self {
        Self {
            net_to_baseline: 39. * FEET_TO_METERS,
            net_to_service_line: 21. * FEET_TO_METERS,
            center_to_sideline: 13.5 * FEET_TO_METERS,
            center_to_alley: 18. * FEET_TO_METERS,
        }
    }
}

impl CourtDimensions {
    fn court_surface_path(&self) -> WorldPolygon {
        let baseline = self.net_to_baseline * Vec3::Z;
        let alley = self.center_to_alley * Vec3::X;
        let corners = [
            -alley + baseline,
            alley + baseline,
            alley - baseline,
            -alley - baseline,
        ]
        .to_vec();
        WorldPolygon { corners }
    }

    fn court_boundaries_path(&self) -> WorldPolyline {
        let baseline = self.net_to_baseline * Vec3::Z;
        let service_line = self.net_to_service_line * Vec3::Z;
        let sideline = self.center_to_sideline * Vec3::X;
        let alley = self.center_to_alley * Vec3::X;
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
        ]
        .to_vec();
        WorldPolyline { segments }
    }
}

fn spawn_court_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert_bundle((Transform::default(), GlobalTransform::default()))
        .insert(CourtDimensions::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle::default(),
                    DrawMode::Fill(FillMode {
                        color: Color::hex("7a9b00").unwrap(),
                        options: FillOptions::DEFAULT,
                    }),
                    Transform::default(),
                ))
                .insert(WorldPosition::default())
                .insert(WorldPolygon {
                    corners: vec![
                        Vec3::new(1000., 0., 28. - 0.1),
                        Vec3::new(1000., 0., -1000.),
                        Vec3::new(-1000., 0., -1000.),
                        Vec3::new(-1000., 0., 28. - 0.1),
                    ],
                });
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle::default(),
                    DrawMode::Fill(FillMode {
                        color: Color::hex("366d00").unwrap(),
                        options: FillOptions::DEFAULT,
                    }),
                    Transform::default(),
                ))
                .insert_bundle((
                    CourtSurface,
                    WorldPosition::default(),
                    WorldPolygon::default(),
                ));
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Rectangle::default(),
                    DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.)),
                    Transform::default(),
                ))
                .insert_bundle((WorldPosition::default(), WorldPolyline::default()))
                .insert(CourtOutline);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("textures/net.png"),
                    ..default()
                })
                .insert(WorldPosition(Vec3::new(0., 0.5, 0.)))
                .insert(WorldPositionSync);
        });
}

#[derive(Component)]
struct CourtOutline;

fn update_court_outline_system(
    mut q_outline: Query<(&mut WorldPolyline, &Parent), With<CourtOutline>>,
    q_court: Query<&CourtDimensions, Changed<CourtDimensions>>,
) {
    for (mut outline, parent) in q_outline.iter_mut() {
        if let Ok(dimensions) = q_court.get(parent.0) {
            *outline = dimensions.court_boundaries_path();
        }
    }
}

#[derive(Component)]
struct CourtSurface;

fn update_court_surface_system(
    mut q_surface: Query<(&mut WorldPolygon, &Parent), With<CourtSurface>>,
    q_court: Query<&CourtDimensions, Changed<CourtDimensions>>,
) {
    for (mut surface, parent) in q_surface.iter_mut() {
        if let Ok(dimensions) = q_court.get(parent.0) {
            *surface = dimensions.court_surface_path();
        }
    }
}
