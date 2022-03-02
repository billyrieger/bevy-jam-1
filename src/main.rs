#![feature(bool_to_option)]
#![feature(try_blocks)]

use bevy::prelude::*;
use bevy_easings::*;
use bevy_rapier3d::prelude::*;

const BG_WIDTH: f32 = 272.0;
const BG_HEIGHT: f32 = 256.0;
const PX_SCALE: f32 = 2.5;

mod game;
mod input;
mod resource;
mod world;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: BG_WIDTH * PX_SCALE,
            height: BG_HEIGHT * PX_SCALE,
            resizable: false,
            decorations: false,
            cursor_visible: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(resource::ResourcePlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(world::WorldPlugin)
        .add_state(GameState::Loading)
        .run();
}

// fn sprite_animation_system(
//     time: Res<Time>,
//     mut query: Query<(
//         &mut TextureAtlasSprite,
//         &SpriteAnimation,
//         &mut SpriteAnimationTimer,
//     )>,
// ) {
//     for (mut sprite, animation, mut timer) in query.iter_mut() {
//         timer.0.tick(time.delta());
//         let mut sum = Duration::ZERO;
//         let (index, flipped) = animation
//             .0
//             .iter()
//             .find_map(|frame| {
//                 sum += frame.duration;
//                 (timer.0.elapsed() <= sum).then_some((frame.sprite_index, frame.flipped))
//             })
//             .expect("overextended animation timer");
//         sprite.index = index;
//         sprite.flip_x = flipped;
//     }
// }
