use super::ball::{BallBouncesSinceHit, GameBall, LastHitBy};
use super::level::PointOverEvent;
use super::world::{CameraView, SyncWorldPosition, WorldPosition, WorldSprite};
use crate::game::player::Player;
use crate::AppState;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier3d::prelude::*;

pub const NET_TO_BASELINE: f32 = 39.;
pub const NET_TO_SERVICE_LINE: f32 = 21.;
pub const COURT_WIDTH_SINGLES: f32 = 27.;
pub const COURT_WIDTH_DOUBLES: f32 = 36.;

pub struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCourtEvent>().add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(court_spawner_system),
        );
    }
}

#[derive(Component)]
struct CourtDimensions {
    baseline_depth: f32,
    service_line_depth: f32,
    singles_width: f32,
    doubles_width: f32,
}

#[derive(Component)]
struct Outline(Path);

fn draw_court_system(camera_view: Res<CameraView>, courts: Query<(&WorldPosition, &CourtDimensions, &mut Outline)>) {
    for (pos, dimensions, outline_path) in courts.iter() {

    }
}

pub struct SpawnCourtEvent;

fn court_spawner_system(
    viewer: Res<CameraView>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnCourtEvent>,
) {
    for _ in events.iter() {
        let corners = [
            Vec3::new(-COURT_WIDTH_SINGLES / 2., 0., NET_TO_BASELINE),
            Vec3::new(-COURT_WIDTH_SINGLES / 2., 0., -NET_TO_BASELINE),
            Vec3::new(COURT_WIDTH_SINGLES / 2., 0., -NET_TO_BASELINE),
            Vec3::new(COURT_WIDTH_SINGLES / 2., 0., NET_TO_BASELINE),
        ]
        .map(|p| viewer.to_screen(p));
        let mut path_builder = PathBuilder::new();
        for (p0, p1) in corners.iter().zip(corners.iter().cycle().skip(1)) {
            path_builder.move_to(*p0);
            path_builder.line_to(*p1);
        }
        let line = path_builder.build();

        commands.spawn_bundle(GeometryBuilder::build_as(
            &line.0,
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 1.)),
            Transform::default(),
        ));

        // commands
        //     .spawn_bundle(SpriteBundle {
        //         texture: asset_server.load("textures/net.png"),
        //         ..Default::default()
        //     })
        //     .insert_bundle((
        //         WorldPosition(Vec3::new(0., Y_NETLINE, 0.)),
        //         SyncWorldPosition,
        //         WorldSprite {
        //             base: Vec2::new(0., -5.5),
        //         },
        //     ));
        commands
            .spawn()
            // .spawn_bundle(SpriteBundle {
            //     texture: asset_server.load("textures/court_grass.png"),
            //     ..Default::default()
            // })
            .with_children(|parent| {
                // floor
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(200.0, 200.0, 10.0).into(),
                    flags: ActiveEvents::CONTACT_EVENTS.into(),
                    position: (Vec3::new(0.0, 0.0, -10.0), Quat::IDENTITY).into(),
                    material: ColliderMaterial {
                        friction: 0.9,
                        restitution: 0.5,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                });
                // wall
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(200.0, 1.0, 200.0).into(),
                    position: (Vec3::new(0.0, 15.0, 0.0), Quat::IDENTITY).into(),
                    material: ColliderMaterial {
                        friction: 0.6,
                        restitution: 0.8,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                });
                // net
                // parent.spawn_bundle(ColliderBundle {
                //     position: (
                //         Vec3::new(X_CENTER_LINE, Y_NETLINE, NET_HEIGHT / 2.0),
                //         Quat::IDENTITY,
                //     )
                //         .into(),
                //     shape: ColliderShape::cuboid(
                //         X_DOUBLES_LINE_RIGHT,
                //         NET_THICKNESS / 2.0,
                //         NET_HEIGHT / 2.0,
                //     )
                //     .into(),
                //     material: ColliderMaterial {
                //         friction: 0.6,
                //         restitution: 0.8,
                //         ..Default::default()
                //     }
                //     .into(),
                //     ..Default::default()
                // });
            });
    }
}

// fn handle_bounces_system(
//     mut contact_events: EventReader<ContactEvent>,
//     mut bounces_counter: ResMut<BallBouncesSinceHit>,
//     ball_query: Query<(&WorldPosition, &LastHitBy), With<GameBall>>,
//     mut point_over_events: EventWriter<PointOverEvent>,
// ) {
//     for ev in contact_events.iter() {
//         match ev {
//             ContactEvent::Started(_, _) => {
//                 bounces_counter.0 += 1;
//                 let double_bounce = bounces_counter.0 == 2;
//                 let (ball_pos, last_hit) = ball_query.single();
//                 let x_min = X_SINGLES_LINE_LEFT;
//                 let x_max = X_SINGLES_LINE_RIGHT;
//                 let y_min = match last_hit.0 {
//                     Player::User => Y_NETLINE,
//                     Player::Opponent => Y_NEAR_BASELINE,
//                 };
//                 let y_max = match last_hit.0 {
//                     Player::User => Y_FAR_BASELINE,
//                     Player::Opponent => Y_NETLINE,
//                 };
//                 let inbounds = ball_pos.0.x >= x_min
//                     && ball_pos.0.x <= x_max
//                     && ball_pos.0.y >= y_min
//                     && ball_pos.0.y <= y_max;
//                 let winner = match (&last_hit.0, inbounds, double_bounce) {
//                     (Player::User, true, false) | (Player::Opponent, true, false) => continue,
//                     (Player::Opponent, false, false) => Player::User,
//                     (Player::User, false, false) => Player::Opponent,
//                     (Player::Opponent, _, true) => Player::Opponent,
//                     (Player::User, _, true) => Player::User,
//                 };
//                 info!("the winner is: {winner:?}");
//                 point_over_events.send(PointOverEvent { winner });
//             }
//             ContactEvent::Stopped(_, _) => {}
//         }
//     }
// }
