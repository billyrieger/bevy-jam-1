use crate::GameState;
use bevy::prelude::*;

pub(crate) struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ResourceHandles>()
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_resources))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_resources));
    }
}

#[derive(Default)]
struct ResourceHandles(Vec<HandleUntyped>);

fn load_resources(mut texture_handles: ResMut<ResourceHandles>, asset_server: Res<AssetServer>) {
    texture_handles.0.extend(
        [
        "ball.png",
        "player_female_dark_blue.png",
        "player_male_light_white.png",
        "court_grass.png",
        ]
        .map(|filename| asset_server.load_untyped(filename)),
    );
}

fn check_resources(
    mut state: ResMut<State<GameState>>,
    resource_handles: ResMut<ResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    let handle_ids = resource_handles.0.iter().map(|handle| handle.id);
    if matches!(
        asset_server.get_group_load_state(handle_ids),
        bevy::asset::LoadState::Loaded
    ) {
        state.set(GameState::InGame).unwrap();
    }
}
