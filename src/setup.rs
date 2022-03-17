use crate::AppState;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier3d::prelude::*;

const FONTS: &[&str] = &["fonts/Press_Start_2P/PressStart2P-Regular.ttf"];
const TEXTURES: &[&str] = &[
    "textures/player.png",
    "textures/opponent.png",
    "textures/net.png",
];

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(bevy_rapier3d::physics::RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default())
            .add_plugin(ShapePlugin)
            .init_resource::<AllResourceHandles>()
            .init_resource::<TextureAtlasHandles>()
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup_system))
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_resource_loading_system),
            );
        #[cfg(not(target = "wasm32"))]
        app.add_system(bevy::input::system::exit_on_esc_system);
    }
}

// ====== Resources ======

#[derive(Default)]
struct AllResourceHandles(Vec<HandleUntyped>);

#[derive(Default)]
pub struct TextureAtlasHandles {
    pub player: Handle<TextureAtlas>,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct UiCamera;

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut resource_handles: ResMut<AllResourceHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);

    resource_handles.0.extend(
        std::iter::empty()
            .chain(FONTS.iter())
            .chain(TEXTURES.iter())
            .map(|&filename| asset_server.load_untyped(filename)),
    );

    let player_texture_handle = asset_server.get_handle("textures/opponent.png");
    let player_texture_atlas =
        TextureAtlas::from_grid(player_texture_handle, Vec2::new(24.0, 24.0), 4, 8);
    texture_atlas_handles.player = texture_atlases.add(player_texture_atlas);
}

fn check_resource_loading_system(
    mut state: ResMut<State<AppState>>,
    resource_handles: Res<AllResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    let handle_ids = resource_handles.0.iter().map(|handle| handle.id);
    if matches!(
        asset_server.get_group_load_state(handle_ids),
        bevy::asset::LoadState::Loaded
    ) {
        state.set(AppState::InGame).unwrap();
    }
}
