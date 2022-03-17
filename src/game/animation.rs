use std::time::Duration;

use bevy::utils::HashMap;

use crate::*;

use super::player::{self, *};

pub(crate) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerFrameData>().add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(advance_animations_system),
        );
    }
}

#[derive(Clone)]
pub struct Frame {
    index: usize,
    duration: Duration,
}

pub struct PlayerFrameData(pub HashMap<player::PlayerState, Animation>);

impl Default for PlayerFrameData {
    fn default() -> Self {
        // TODO: find a better way to store the frame data. Tweaking the animations in this format
        // is arduous. Is there a third-party plugin to work with spritesheet animations?
        let data = HashMap::from_iter([
            (
                PlayerState::ServeReady,
                Animation::new([(0, u64::MAX)], false),
            ),
            (
                PlayerState::ServeToss,
                Animation::new([(1, u64::MAX)], false),
            ),
            (
                PlayerState::ServeHit,
                Animation::new([(2, 50), (3, 300)], false),
            ),
            (
                PlayerState::Idle,
                Animation::new([(4, 300), (5, 100), (6, 200), (7, 100)], true),
            ),
            (
                PlayerState::Run,
                Animation::new([(11, 150), (8, 75), (9, 150), (10, 75)], true),
            ),
        ]);
        Self(data)
    }
}

#[derive(Component, Clone, Default)]
pub struct Animation {
    frames: Vec<Frame>,
    timer: Timer,
}

impl Animation {
    // usize is frame index, u64 is frame length in milliseconds
    pub fn new(frame_data: impl IntoIterator<Item = (usize, u64)>, repeating: bool) -> Self {
        let frames: Vec<Frame> = frame_data
            .into_iter()
            .map(|(i, duration)| Frame {
                index: i,
                duration: Duration::from_millis(duration),
            })
            .collect();
        let total_duration = frames.iter().fold(Duration::ZERO, |acc, frame| {
            acc.saturating_add(frame.duration)
        });
        let timer = Timer::new(total_duration, repeating);
        Self { frames, timer }
    }
}

fn advance_animations_system(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());
        let mut sum = Duration::ZERO;
        sprite.index = animation
            .frames
            .iter()
            .find(|frame| {
                sum = sum.saturating_add(frame.duration);
                sum >= animation.timer.elapsed()
            })
            .map(|frame| frame.index)
            .unwrap_or(animation.frames.last().expect("no frames!").index);
    }
}

// impl SpriteAnimation {
//     pub(crate) fn new<const N: usize>(
//         indices: [usize; N],
//         durations: [f32; N],
//         repeating: bool,
//     ) -> Self {
//         let frames = indices
//             .into_iter()
//             .zip(durations)
//             .map(|(index, duration)| SpriteAnimationFrame {
//                 sprite_index: index,
//                 duration: Duration::from_secs_f32(duration),
//             })
//             .collect();
//         Self {
//             frames,
//             timer: Timer::from_seconds(durations.iter().sum(), repeating),
//         }
//     }

//     pub(crate) fn player_serve() -> Self {
//         Self::new([0, 1, 2, 3], [1.0, 0.3, 0.2, 0.2], false)
//     }

//     pub(crate) fn player_idle() -> Self {
//         Self::new([4, 5, 6, 7], [0.3, 0.1, 0.2, 0.1], true)
//     }

//     pub(crate) fn player_run() -> Self {
//         // The spritesheet frames are off by one for this animation.
//         Self::new([9, 10, 11, 8], [0.2, 0.2, 0.2, 0.2], true)
//     }

//     pub(crate) fn player_charge() -> Self {
//         Self::new([12], [0.1], true)
//     }

//     pub(crate) fn player_swing() -> Self {
//         Self::new([13, 14], [0.05, 0.2], false)
//     }

//     pub(crate) fn opponent_idle() -> Self {
//         Self::new([20, 21, 22, 23], [0.3, 0.1, 0.2, 0.1], true)
//     }

//     pub(crate) fn opponent_run() -> Self {
//         // The spritesheet frames are off by one for this animation.
//         Self::new([25, 26, 27, 24], [0.2, 0.2, 0.2, 0.2], true)
//     }

//     pub(crate) fn opponent_charge() -> Self {
//         Self::new([28], [0.1], true)
//     }

//     pub(crate) fn opponent_swing() -> Self {
//         Self::new([28, 29, 30], [0.1, 0.05, 0.2], false)
//     }
// }
