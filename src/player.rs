use core::time::Duration;

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

use benimator::FrameRate;

const ANIMATION_DURATION: u64 = 200;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Standing,
}

#[derive(Component, Deref)]
struct PlayerAnimation(benimator::Animation);


#[derive(Default, Component, Deref, DerefMut)]
struct PlayerAnimationState(benimator::State);

#[derive(Resource, Default)]
struct PlayerSpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Resource, Default)]
struct DirectionAnimations {
    up: benimator::Animation,
    down: benimator::Animation,
    left: benimator::Animation,
    right: benimator::Animation,
}

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, prepare_player_animations)
            .add_startup_system(setup)
            .add_system(move_player);
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
    time: Res<Time>,
    mut query: Query<(&mut Handle<SpriteSheetAnimation>, &mut Transform)>,
) {
    let (mut animation, mut transform) = query.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        *animation = animations.left.clone();
        transform.translation.x -= 100. * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::D) {
        *animation = animations.right.clone();
        transform.translation.x += 100. * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::S) {
        *animation = animations.down.clone();
        transform.translation.y -= 100. * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::W) {            
        *animation = animations.up.clone();
        transform.translation.y += 100. * time.delta_seconds();
    } else if keyboard_input.any_just_released([KeyCode::A, KeyCode::S, KeyCode::D, KeyCode::W]) {
        *animations = animations.standing.clone();
    }
}

fn prepare_player_animations(
    mut direction_animations: ResMut<DirectionAnimations>,
    asset_server: Res<AssetServer>
) {
    direction_animations.up = Animation(benimator::Animation::from_indices(9..=11, FrameRate::from_total_duration(ANIMATION_DURATION)));
    direction_animations.down = Animation(benimator::Animation::from_indices(0..=2, FrameRate::from_total_duration(ANIMATION_DURATION)));
    direction_animations.left = Animation(benimator::Animation::from_indices(3..=5, FrameRate::from_total_duration(ANIMATION_DURATION)));
    direction_animations.right = Animation(benimator::Animation::from_indices(6..=8, FrameRate::from_total_duration(ANIMATION_DURATION)));    
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    direction_animations: Res<DirectionAnimations>,
) {
    let texture_handle = asset_server.load("Male 01-1.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(32.0, 32.0), 
        3, 
        4,
        None,
        None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas); 

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(PlayerAnimation::direction_animations.left)
        .insert(PlayerAnimationState::default())
        .insert(Direction::Left);
}