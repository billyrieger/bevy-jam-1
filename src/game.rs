use crate::*;

mod animation;
mod ball;
mod court;
mod player;
mod level;
mod world;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(world::WorldPlugin)
            .add_plugin(level::LevelPlugin)
            .add_plugin(ball::BallPlugin)
            .add_plugin(court::CourtPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(animation::AnimationPlugin);
    }
}
