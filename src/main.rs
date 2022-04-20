use std::time::Duration;
use core::ops::Deref;

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use benimator::{AnimationPlugin, *};

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
struct PlayerAnimations {
    left: Handle<SpriteSheetAnimation>,
    right: Handle<SpriteSheetAnimation>,
    up: Handle<SpriteSheetAnimation>,
    down: Handle<SpriteSheetAnimation>,
}

fn main() {
    App::new()
        .init_resource::<PlayerAnimations>()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationPlugin::default())
        .add_startup_system_to_stage(StartupStage::PreStartup, create_player_animations)
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
    mut query: Query<&mut Handle<SpriteSheetAnimation>>,
) {
    let mut animation = query.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        *animation = animations.left.clone();
    } else if keyboard_input.pressed(KeyCode::D) {
        *animation = animations.right.clone();
    } else if keyboard_input.pressed(KeyCode::S) {
        *animation = animations.down.clone();
    } else if keyboard_input.pressed(KeyCode::W) {            
        *animation = animations.up.clone();
    }
}

fn create_player_animations(
    mut handles: ResMut<PlayerAnimations>,
    mut assets: ResMut<Assets<SpriteSheetAnimation>>,
) {
    handles.right = assets.add(SpriteSheetAnimation::from_range(6..=8, Duration::from_millis(100)));
    handles.left = assets.add(SpriteSheetAnimation::from_range(3..=5, Duration::from_millis(100)));
    handles.up = assets.add(SpriteSheetAnimation::from_range(9..=11, Duration::from_millis(100)));
    handles.down = assets.add(SpriteSheetAnimation::from_range(0..=2, Duration::from_millis(100)));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animations: Res<PlayerAnimations>
) {
    let texture_handle = asset_server.load("Male 01-1.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(animations.left.clone())
        .insert(Play)
        .insert(Direction::Left);
}