use std::time::Duration;

use bevy::{asset::LoadState, prelude::*};
use bevy_easings::*;

fn main() {
    App::new()
        .add_state(GameState::Loading)
        .init_resource::<ResourceHandles>()
        .insert_resource(WindowDescriptor {
            width: 272.0 * 3.0,
            height: 256.0 * 3.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_resources))
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_resources))
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(sprite_animation)
                .with_system(sync_mouse_position)
                .with_system(move_player_to_mouse)
                .with_system(update_player_animation),
        )
        .run();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    InGame,
}

// ====== Resources ======

#[derive(Default)]
struct ResourceHandles {
    textures: Vec<HandleUntyped>,
}

// ====== Components ======

#[derive(Component)]
struct MainCamera;

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

#[derive(Component)]
enum PlayerAnimationState {
    Idle,
    Run,
}

#[derive(Component)]
struct SpriteAnimation {
    atlas_indices: Vec<usize>,
    durations: Vec<Duration>,
    timer: Timer,
}

impl SpriteAnimation {
    fn player_idle_north() -> Self {
        let durations = [0.5; 4].map(Duration::from_secs_f32).to_vec();
        Self {
            atlas_indices: [16, 17, 18, 19].into(),
            timer: Timer::new(durations.iter().sum(), true),
            durations,
        }
    }

    fn player_run_north() -> Self {
        let durations = [0.2; 4].map(Duration::from_secs_f32).to_vec();
        Self {
            atlas_indices: [8, 9, 10, 11].into(),
            timer: Timer::new(durations.iter().sum(), true),
            durations,
        }
    }
}

// ====== Systems ======

fn sprite_animation(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut SpriteAnimation)>,
) {
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());
        let mut sum = Duration::ZERO;
        for (&i, &duration) in animation.atlas_indices.iter().zip(&animation.durations) {
            sum += duration;
            if sum >= animation.timer.elapsed() {
                sprite.index = i;
                break;
            }
        }
    }
}

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
        state.set(GameState::InGame).unwrap();
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let scale = Transform::from_scale(Vec3::splat(3.0));

    let texture_handle = asset_server.get_handle("textures/player_ashe_white.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle,
            transform: scale.with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .insert(PlayerAnimationState::Run)
        .insert(SpriteAnimation::player_run_north());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("textures/court_grass.png"),
            transform: scale,
            ..Default::default()
        })
        .insert(Court {
            surface: Surface::Grass,
        });

    commands.spawn().insert(MousePosition(Vec2::ZERO));
}

#[derive(Component)]
struct MousePosition(Vec2);

fn sync_mouse_position(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_mouse: Query<&mut MousePosition>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    let mut mouse_pos = q_mouse.single_mut();

    // get the window that the camera is displaying to
    let wnd = wnds.get(camera.window).unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        mouse_pos.0 = world_pos;
    }
}

fn update_player_animation(
    mut query: Query<(&mut SpriteAnimation, &PlayerAnimationState), Changed<PlayerAnimationState>>,
) {
    for (mut anim, anim_state) in query.iter_mut() {
        *anim = match anim_state {
            PlayerAnimationState::Idle => SpriteAnimation::player_idle_north(),
            PlayerAnimationState::Run => SpriteAnimation::player_run_north(),
        };
    }
}

fn move_player_to_mouse(
    mut player_query: Query<(&mut Transform, &mut PlayerAnimationState)>,
    mouse_query: Query<&MousePosition>,
) {
    if let Ok(mouse_pos) = mouse_query.get_single() {
        for (mut transform, mut anim_state) in player_query.iter_mut() {
            let speed = 0.05;
            if (transform.translation.truncate() - mouse_pos.0).length() > 1.0 {
                transform.translation =
                    speed * mouse_pos.0.extend(1.0) + (1.0 - speed) * transform.translation;
                if !matches!(*anim_state, PlayerAnimationState::Run) {
                    *anim_state = PlayerAnimationState::Run;
                }
            } else if !matches!(*anim_state, PlayerAnimationState::Idle) {
                *anim_state = PlayerAnimationState::Idle;
            }
        }
    }
}
