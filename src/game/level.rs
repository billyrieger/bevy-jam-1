use crate::*;

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_scene))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(reset_scene_system)
                    .with_system(clear_scene_system)
                    .with_system(update_score_system),
            );
    }
}

fn update_score_system(
    mut events: EventReader<PointOverEvent>,
    mut game_overs: EventWriter<GameOverEvent>,
    mut user_score: ResMut<UserScore>,
    mut opponent_score: ResMut<OpponentScore>,
    mut results_text: Query<&mut Text, With<ResultsText>>,
) {
    for ev in events.iter() {
        match ev.winner {
            Player::User => user_score.0 += 1,
            Player::Opponent => opponent_score.0 += 1,
        }
    }
    let text = &mut results_text.single_mut().sections[0].value;
    if user_score.0 >= 7 && user_score.0 >= opponent_score.0 + 2 {
        *text = "You won!".to_owned();
        game_overs.send(GameOverEvent);
    } else if opponent_score.0 >= 7 && opponent_score.0 >= user_score.0 + 2 {
        *text = "You lost!".to_owned();
        game_overs.send(GameOverEvent);
    }
}

fn clear_scene_system(
    mut commands: Commands,
    mut events: EventReader<GameOverEvent>,
    query: Query<Entity, Or<(With<Player>, With<Shadow>, With<GameBall>)>>,
) {
    for _ in events.iter() {
        for id in query.iter() {
            commands.entity(id).despawn();
        }
    }
}

fn reset_scene_system(
    mut commands: Commands,
    mut events: EventReader<PointOverEvent>,
    ball_query: Query<Entity, Or<(With<GameBall>, With<GameBallShadow>)>>,
    mut ball_events: EventWriter<SpawnBallEvent>,
    mut bounces: ResMut<BallBouncesSinceHit>,
) {
    for _ in events.iter() {
        for id in ball_query.iter() {
            commands.entity(id).despawn();
        }
        bounces.0 = 0;
        ball_events.send(SpawnBallEvent {
            position: WorldPosition(Vec3::new(X_CENTER_LINE, Y_NEAR_BASELINE, 3.0)),
            velocity: RigidBodyVelocity {
                linvel: Vec3::new(10. * rand::random::<f32>() - 5., 15., 10.).into(),
                ..Default::default()
            },
        });
    }
}

fn setup_scene(
    mut ball_events: EventWriter<SpawnBallEvent>,
    mut court_events: EventWriter<SpawnCourtEvent>,
    mut player_events: EventWriter<SpawnPlayerEvent>,
) {
    ball_events.send(SpawnBallEvent {
        position: WorldPosition(Vec3::new(X_CENTER_LINE, Y_NEAR_BASELINE, 3.0)),
        velocity: RigidBodyVelocity {
            linvel: Vec3::new(10. * rand::random::<f32>() - 5., 15., 10.).into(),
            ..Default::default()
        },
    });
    court_events.send(SpawnCourtEvent);
    player_events.send(SpawnPlayerEvent {
        position: WorldPosition(Vec3::new(0.0, Y_NEAR_BASELINE - 1.0, 0.0)),
        opponent: false,
    });
    player_events.send(SpawnPlayerEvent {
        position: WorldPosition(Vec3::new(0.0, Y_FAR_BASELINE + 1.0, 0.0)),
        opponent: true,
    });
}
