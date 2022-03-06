use crate::AppState;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier3d::prelude::*;

const FONTS: &[&str] = &["fonts/Press_Start_2P/PressStart2P-Regular.ttf"];
const TEXTURES: &[&str] = &["textures/player.png"];

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, mut app: &mut App) {
        app = app
            .add_plugins(DefaultPlugins)
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(bevy_rapier3d::physics::RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::default())
            .add_plugin(ShapePlugin)
            .init_resource::<ResourceHandles>()
            .add_system(bevy::input::system::exit_on_esc_system)
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_resource_loading),
            );
        #[cfg(not(target = "wasm32"))]
        app.add_system(bevy::input::system::exit_on_esc_system);
    }
}

// ====== Resources ======

#[derive(Default)]
struct ResourceHandles(Vec<HandleUntyped>);

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct UiCamera;

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut texture_handles: ResMut<ResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
    // rapier_config.gravity = Vec3::new(0.0, -15.0, ).into();

    let textures = [
        "textures/ball.png",
        "textures/player.png",
        "textures/opponent.png",
        "textures/court_grass.png",
        "textures/net.png",
    ];
    texture_handles.0.extend(
        std::iter::empty()
            .chain(FONTS.into_iter().copied())
            .chain(textures)
            .map(|filename| asset_server.load_untyped(filename)),
    );
}

fn check_resource_loading(
    mut state: ResMut<State<AppState>>,
    resource_handles: ResMut<ResourceHandles>,
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
