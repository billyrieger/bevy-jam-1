use crate::*;

pub(crate) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(EasingsPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .init_resource::<ResourceHandles>()
            .init_resource::<GameAssets>()
            .add_event::<SpawnBallEvent>()
            .add_event::<SpawnCourtEvent>()
            .add_event::<SpawnPlayerEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_resource_loading),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut texture_handles: ResMut<ResourceHandles>,
    mut game_assets: ResMut<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
    rapier_config.gravity = Vec3::new(0.0, 0.0, -15.0).into();

    game_assets.ball_texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.get_handle("textures/ball.png"),
        Vec2::new(8.0, 8.0),
        1,
        6,
    ));

    game_assets.court_texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.get_handle("textures/court_grass.png"),
        Vec2::new(16.0, 16.0),
        17,
        16,
    ));

    let fonts = ["fonts/Press_Start_2P/PressStart2P-Regular.ttf"];
    let textures = [
        "textures/ball.png",
        "textures/player.png",
        "textures/court_grass.png",
    ];
    texture_handles.0.extend(
        std::iter::empty()
            .chain(fonts)
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
