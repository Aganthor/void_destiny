use core::time::Duration;

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    render::texture,
};

use benimator::*;

const ANIMATION_DURATION: u64 = 200;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Standing,
}

#[derive(Component, Deref, Debug)]
struct PlayerAnimation(benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
struct PlayerAnimationState(benimator::State);

#[derive(Resource)]
struct DirectionAnimations {
    up: benimator::Animation,
    down: benimator::Animation,
    left: benimator::Animation,
    right: benimator::Animation,
}

impl Default for DirectionAnimations {
    fn default() -> Self {
        let anim_duration = Duration::new(ANIMATION_DURATION, 0);
        DirectionAnimations {
            up: Animation::from_indices(9..=11, FrameRate::from_frame_duration(anim_duration)),
            down: Animation::from_indices(0..=2, FrameRate::from_frame_duration(anim_duration)),
            left: Animation::from_indices(3..=5, FrameRate::from_frame_duration(anim_duration)),
            right: Animation::from_indices(6..=8, FrameRate::from_frame_duration(anim_duration)),
        }
    }
}

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DirectionAnimations>()
            .add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_system(move_player)
                    .with_system(animate_player.after(move_player)),
            );
    }
}

fn animate_player(
    time: Res<Time>,
    mut animation_query: Query<(
        &PlayerAnimation,
        &mut PlayerAnimationState,
        &mut TextureAtlasSprite,
    )>,
) {
    for (animation, mut animation_state, mut texture_atlas) in &mut animation_query {
        animation_state.update(animation, time.delta());
        texture_atlas.index = animation_state.frame_index();
        println!("Texture index in atlas = {}", texture_atlas.index);
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<DirectionAnimations>,
    time: Res<Time>,
    mut direction_query: Query<(&mut PlayerAnimation, &mut Transform)>,
) {
    let (mut animation, mut transform) = direction_query.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        *animation = PlayerAnimation(animations.left.clone());
        transform.translation.x -= 100. * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::D) {
        *animation = PlayerAnimation(animations.right.clone());
        transform.translation.x += 100. * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::S) {
        *animation = PlayerAnimation(animations.down.clone());
        transform.translation.y -= 100. * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::W) {
        *animation = PlayerAnimation(animations.up.clone());
        transform.translation.y += 100. * time.delta_seconds();
    } else if keyboard_input.any_just_released([KeyCode::A, KeyCode::S, KeyCode::D, KeyCode::W]) {
        //*animations = animations.standing.clone();
        println!("Just released a key... stop animation.")
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    direction_animations: Res<DirectionAnimations>,
) {
    let texture_handle = asset_server.load("Male 01-1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(PlayerAnimation(direction_animations.left.clone()))
        .insert(PlayerAnimationState::default())
        .insert(Direction::Left);
}
