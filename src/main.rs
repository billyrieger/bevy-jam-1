use std::time::Duration;

use bevy::{asset::LoadState, prelude::*};

fn main() {
    App::new()
        .add_state(GameState::Loading)
        .init_resource::<ResourceHandles>()
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_resources))
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_resources))
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(load_main_menu))
        .run();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    MainMenu,
}

// ====== Resources ======

#[derive(Default)]
struct ResourceHandles {
    textures: Vec<HandleUntyped>,
}

// ====== Components ======

#[derive(Component)]
struct Court {
    surface: Surface,
}

enum Surface {
    AcrylicBlue,
    AcrylicGreen,
    Clay,
    Grass,
    Concrete,
}

struct Player;

enum Facing {
    Left,
    Right,
}

enum PlayerState {
    Hit,
    Idle,
    Run,
    Serve,
    Smash,
}

// Opponents

// Unfair advantage: improves as point goes on.
#[derive(Component)]
struct ArthurAshe;

// Unfair advantage: speedy.
struct BillieJeanKing;

// Unfair advantage: fast serve.
#[derive(Component)]
struct PeteSampras;

// Unfair advantage: shot placement.
#[derive(Component)]
struct SerenaWilliams;

struct Animation {
    texture_atlas_indices: Vec<usize>,
    frame_durations: Vec<Duration>,
    repeating: bool,
}

// ====== Systems ======

fn load_resources(mut texture_handles: ResMut<ResourceHandles>, asset_server: Res<AssetServer>) {
    texture_handles.textures = asset_server.load_folder("textures").unwrap();
}

fn check_resources(
    mut state: ResMut<State<GameState>>,
    texture_handles: ResMut<ResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    if matches!(
        asset_server.get_group_load_state(texture_handles.textures.iter().map(|handle| handle.id)),
        LoadState::Loaded
    ) {
        state.set(GameState::MainMenu).unwrap();
    }
}

fn load_main_menu() {
    println!("main menu!");
}
