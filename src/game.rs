use crate::*;

mod animation;
mod input;
mod player;
mod spawn;
mod world;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(input::InputPlugin)
            .add_plugin(world::WorldPlugin)
            .add_plugin(spawn::SpawnPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(animation::AnimationPlugin);
    }
}
