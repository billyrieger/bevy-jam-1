use crate::*;

pub(crate) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(sprite_animation));
    }
}

fn sprite_animation(
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
            .expect("overextended animation timer")
            .sprite_index;
    }
}

impl SpriteAnimation {
    fn new<const N: usize>(indices: [usize; N], durations: [f32; N]) -> Self {
        let frames = indices
            .into_iter()
            .zip(durations)
            .map(|(index, duration)| SpriteAnimationFrame {
                sprite_index: index,
                duration: Duration::from_secs_f32(duration),
            })
            .collect();
        Self {
            frames,
            timer: Timer::from_seconds(durations.iter().sum(), true),
        }
    }

    pub(crate) fn player_idle() -> Self {
        Self::new([16, 17, 18, 19], [0.3, 0.1, 0.2, 0.1])
    }
}
