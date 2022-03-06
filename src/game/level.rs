use crate::AppState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::ball::{SpawnBallEvent, BallBouncesSinceHit, GameBall, GameBallShadow};
use super::court::{SpawnCourtEvent};
use super::player::{SpawnPlayerEvent, Player};
use super::ui::ResultsText;
use super::world::{WorldPosition, Shadow};
pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UserScore>()
            .init_resource::<OpponentScore>()
            .add_event::<PointOverEvent>()
            .add_event::<GameOverEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_scene))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    // .with_system(reset_scene_system)
                    .with_system(clear_scene_system)
                    .with_system(update_score_system),
            );
    }
}

// Resources

#[derive(Default)]
pub struct UserScore(pub u32);

#[derive(Default)]
pub struct OpponentScore(pub u32);

// ====== Events ======

pub struct PointOverEvent {
    pub winner: Player,
}

pub struct GameOverEvent;

// ====== Systems ======

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

fn setup_scene(
    // mut ball_events: EventWriter<SpawnBallEvent>,
    mut court_events: EventWriter<SpawnCourtEvent>,
    mut player_events: EventWriter<SpawnPlayerEvent>,
) {
    court_events.send(SpawnCourtEvent);
    player_events.send(SpawnPlayerEvent {
        position: WorldPosition(Vec3::new(0., 0., 0.)),
        opponent: false,
    });
}
