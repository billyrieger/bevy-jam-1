use crate::*;

pub(crate) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(advance_animations)
                .with_system(update_current_player_animation)
                .with_system(change_player_state),
        );
    }
}

fn advance_animations(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());
        let mut sum = Duration::ZERO;
        sprite.index = animation
            .frames
            .iter()
            .find(|frame| {
                sum += frame.duration;
                sum >= animation.timer.elapsed()
            })
            .map(|frame| frame.sprite_index)
            .unwrap_or(0);
    }
}

fn update_current_player_animation(
    mut player_query: Query<(&mut SpriteAnimation, &PlayerState), Changed<PlayerState>>,
) {
    for (mut animation, player_state) in player_query.iter_mut() {
        *animation = match player_state{
            PlayerState::Idle => SpriteAnimation::player_idle(),
            PlayerState::Run => SpriteAnimation::player_run(),
            PlayerState::Swing => SpriteAnimation::player_swing(),
        }
    }
}

fn change_player_state(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut PlayerState, &mut SpriteAnimation, &NextPlayerState)>,
) {
    for (entity, mut player_state, mut animation, next_state) in player_query.iter_mut() {
        if animation.timer.just_finished() && !animation.timer.repeating() {
            *player_state = next_state.0.clone();
            *animation = match *player_state {
                PlayerState::Idle => SpriteAnimation::player_idle(),
                PlayerState::Run => SpriteAnimation::player_run(),
                PlayerState::Swing => SpriteAnimation::player_swing(),
            };
            commands.entity(entity).remove::<NextPlayerState>();
        }
    }
}
