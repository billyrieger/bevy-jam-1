use crate::*;

pub(crate) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(advance_animations));
    }
}

impl SpriteAnimation {
    pub(crate) fn new<const N: usize>(
        indices: [usize; N],
        durations: [f32; N],
        repeating: bool,
    ) -> Self {
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
            timer: Timer::from_seconds(durations.iter().sum(), repeating),
        }
    }

    pub(crate) fn player_serve() -> Self {
        Self::new([0, 1, 2, 3], [1.0, 0.3, 0.2, 0.2], false)
    }

    pub(crate) fn player_idle() -> Self {
        Self::new([4, 5, 6, 7], [0.3, 0.1, 0.2, 0.1], true)
    }

    pub(crate) fn player_run() -> Self {
        // The spritesheet frames are off by one for this animation.
        Self::new([9, 10, 11, 8], [0.2, 0.2, 0.2, 0.2], true)
    }

    pub(crate) fn player_charge() -> Self {
        Self::new([12], [0.1], true)
    }

    pub(crate) fn player_swing() -> Self {
        Self::new([13, 14], [0.05, 0.2], false)
    }

    pub(crate) fn opponent_idle() -> Self {
        Self::new([20, 21, 22, 23], [0.3, 0.1, 0.2, 0.1], true)
    }

    pub(crate) fn opponent_run() -> Self {
        // The spritesheet frames are off by one for this animation.
        Self::new([25, 26, 27, 24], [0.2, 0.2, 0.2, 0.2], true)
    }

    pub(crate) fn opponent_charge() -> Self {
        Self::new([28], [0.1], true)
    }

    pub(crate) fn opponent_swing() -> Self {
        Self::new([28, 29, 30], [0.1, 0.05, 0.2], false)
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
            .unwrap_or(animation.frames.last().expect("no frames!").sprite_index);
    }
}
