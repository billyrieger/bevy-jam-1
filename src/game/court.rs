use crate::*;

pub(crate) struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(court_spawner_system)
                .with_system(handle_bounces_system),
        );
    }
}

fn handle_bounces_system(
    mut contact_events: EventReader<ContactEvent>,
    mut bounces_counter: ResMut<BallBouncesSinceHit>,
    ball_query: Query<(&WorldPosition, &LastHitBy), With<GameBall>>,
    mut point_over_events: EventWriter<PointOverEvent>,
) {
    for ev in contact_events.iter() {
        match ev {
            ContactEvent::Started(_, _) => {
                bounces_counter.0 += 1;
                let double_bounce = bounces_counter.0 == 2;
                let (ball_pos, last_hit) = ball_query.single();
                let x_min = X_SINGLES_LINE_LEFT;
                let x_max = X_SINGLES_LINE_RIGHT;
                let y_min = match last_hit.0 {
                    Player::User => Y_NETLINE,
                    Player::Opponent => Y_NEAR_BASELINE,
                };
                let y_max = match last_hit.0 {
                    Player::User => Y_FAR_BASELINE,
                    Player::Opponent => Y_NETLINE,
                };
                dbg!(ball_pos);
                dbg!((x_min, x_max));
                dbg!((y_min, y_max));
                let inbounds = ball_pos.0.x >= x_min
                    && ball_pos.0.x <= x_max
                    && ball_pos.0.y >= y_min
                    && ball_pos.0.y <= y_max;
                dbg!(inbounds);
                let winner = match (&last_hit.0, inbounds, double_bounce) {
                    (Player::User, true, false) | (Player::Opponent, true, false) => continue,
                    (Player::Opponent, false, false) => Player::User,
                    (Player::User, false, false) => Player::Opponent,
                    (Player::Opponent, _, true) => Player::Opponent,
                    (Player::User, _, true) => Player::User,
                };
                info!("the winner is: {winner:?}");
                point_over_events.send(PointOverEvent { winner });
            }
            ContactEvent::Stopped(_, _) => {}
        }
    }
}

fn court_spawner_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnCourtEvent>,
) {
    for _ in events.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("textures/net.png"),
                transform: Transform::from_scale(Vec3::splat(PX_SCALE)),
                ..Default::default()
            })
            .insert_bundle((
                WorldPosition(Vec3::new(0.0, Y_NETLINE, 0.0)),
                SyncWorldPosition,
                WorldSprite {
                    base: Vec2::new(0.0, -22.0),
                },
            ));
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("textures/court_grass.png"),
                transform: Transform::from_scale(Vec3::splat(PX_SCALE)),
                ..Default::default()
            })
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
                parent.spawn_bundle(ColliderBundle {
                    position: (
                        Vec3::new(X_CENTER_LINE, Y_NETLINE, NET_HEIGHT / 2.0),
                        Quat::IDENTITY,
                    )
                        .into(),
                    shape: ColliderShape::cuboid(
                        X_DOUBLES_LINE_RIGHT,
                        NET_THICKNESS / 2.0,
                        NET_HEIGHT / 2.0,
                    )
                    .into(),
                    material: ColliderMaterial {
                        friction: 0.6,
                        restitution: 0.8,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                });
            });
    }
}
