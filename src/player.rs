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


#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerAnimations>()
            .add_startup_system_to_stage(StartupStage::PreStartup, load_player_texture)
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

fn create_player_animations(
    mut player_animation_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>
) {
    //player_animation_handles.handles = asset_server.load("")
    //player_animations.right = Animation(benimator::Animation::from_indices(6..=8, FrameRate::from_total_duration(ANIMATION_DURATION)));
    // handles.right = assets.add(SpriteSheetAnimation::from_range(6..=8, Duration::from_millis(ANIMATION_DURATION)));
    // handles.left = assets.add(SpriteSheetAnimation::from_range(3..=5, Duration::from_millis(ANIMATION_DURATION)));
    // handles.up = assets.add(SpriteSheetAnimation::from_range(9..=11, Duration::from_millis(ANIMATION_DURATION)));
    // handles.down = assets.add(SpriteSheetAnimation::from_range(0..=2, Duration::from_millis(ANIMATION_DURATION)));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Male 01-1.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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