use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::{
    input::{ButtonInput, keyboard::KeyCode},
    prelude::*,
};
use bevy_spritesheet_animation::prelude::*;

use crate::events::{MoveEvent, MoveLegal};

use crate::states::GameState;

const MOVE_SPEED: f32 = 20.0;
const PLAYER_TILE_SIZE: f32 = 32.0;

//#[derive(Component, Inspectable)]
#[derive(Component)]
pub struct Player {
    speed: f32,
    size: f32,
}

#[derive(Resource)]
struct PlayerAnimations {
    run_right: Handle<Animation>,
    run_left: Handle<Animation>,
    run_up: Handle<Animation>,
    run_down: Handle<Animation>,
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpritesheetAnimationPlugin::default())
            .add_systems(Startup, spawn_caracter)
            //.add_systems(PreUpdate, try_move_player)
            .add_systems(Update, (move_player, update_camera).chain())
            .add_systems(Update, zoom_map.run_if(in_state(GameState::GameRunning)));
    }
}

// fn try_move_player(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     library: Res<AnimationLibrary>,
//     time: Res<Time>,
//     mut player_query: Query<(&mut Sprite, &mut SpritesheetAnimation, &Transform, &Player)>,
//     mut move_event: MessageWriter<MoveEvent>,
// ) {
//     let mut direction = Vec3::ZERO;
//     let Ok((mut _sprite, mut animation, player_transform, player)) = player_query.single_mut()
//     else {
//         return;
//     };
//     let mut player_move_event = MoveEvent {
//         origin: Some(player_transform.translation),
//         destination: None,
//     };
//     let mut send_event = false;
//     let mut destination = player_transform.translation;

//     if keyboard.pressed(KeyCode::KeyA) {
//         if let Some(run_animation_id) = library.animation_with_name("run_left") {
//             if animation.animation_id != run_animation_id {
//                 animation.switch(run_animation_id);
//             }
//         }
//         destination.x -= player.speed * player.size * time.delta_secs();
//         send_event = true;
//         direction -= Vec3::new(1.0, 0.0, 0.0);
//     } else if keyboard.pressed(KeyCode::KeyD) {
//         if let Some(run_animation_id) = library.animation_with_name("run_right") {
//             if animation.animation_id != run_animation_id {
//                 animation.switch(run_animation_id);
//             }
//         }
//         destination.x += player.speed * player.size * time.delta_secs();
//         send_event = true;
//         direction += Vec3::new(1.0, 0.0, 0.0);
//     } else if keyboard.pressed(KeyCode::KeyS) {
//         if let Some(run_animation_id) = library.animation_with_name("run_down") {
//             if animation.animation_id != run_animation_id {
//                 animation.switch(run_animation_id);
//             }
//         }
//         destination.y -= player.speed * player.size * time.delta_secs();
//         send_event = true;
//         direction -= Vec3::new(0.0, 1.0, 0.0);
//     } else if keyboard.pressed(KeyCode::KeyW) {
//         if let Some(run_animation_id) = library.animation_with_name("run_up") {
//             if animation.animation_id != run_animation_id {
//                 animation.switch(run_animation_id);
//             }
//         }
//         destination.y += player.speed * player.size * time.delta_secs();
//         send_event = true;
//         direction += Vec3::new(0.0, 1.0, 0.0);
//     }
//     if keyboard.any_just_released([KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD, KeyCode::KeyW]) {
//         send_event = false;
//         // Need to create an idle animation!
//     }

//     if send_event {
//         player_move_event.destination = Some(destination);
//         move_event.write(player_move_event);
//     }
// }

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    const CAMERA_DECAY_RATE: f32 = 2.; // Adjust this value to control the smoothness
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn spawn_caracter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    commands.spawn((Camera2d::default(), PlayerCamera));

    let image = asset_server.load("Male 01-1.png");

    let spritesheet = Spritesheet::new(&image, 3, 4);

    // Move right
    let run_right_animation = spritesheet
        .create_animation()
        .add_row(2)
        .set_duration(AnimationDuration::PerFrame(100))
        .build();
    let run_right_anim_handle = animations.add(run_right_animation);

    // Move left
    let run_left_animation = spritesheet
        .create_animation()
        .add_row(1)
        .set_duration(AnimationDuration::PerFrame(100))
        .build();
    let run_left_anim_handle = animations.add(run_left_animation);

    // Move down
    let run_down_animation = spritesheet
        .create_animation()
        .add_row(0)
        .set_duration(AnimationDuration::PerFrame(100))
        .build();
    let run_down_anim_handle = animations.add(run_down_animation);

    // Move up
    let run_up_animation = spritesheet
        .create_animation()
        .add_row(3)
        .set_duration(AnimationDuration::PerFrame(100))
        .build();
    let run_up_anim_handle = animations.add(run_up_animation);

    // Store the animation resource.
    commands.insert_resource(PlayerAnimations {
        run_down: run_down_anim_handle.clone(),
        run_left: run_left_anim_handle,
        run_right: run_right_anim_handle,
        run_up: run_up_anim_handle,
    });

    // // Spawn the player sprite with the animations
    // let atlas = TextureAtlas {
    //     layout: atlas_layouts.add(Spritesheet::new(3, 4).atlas_layout(32, 32)),
    //     ..default()
    // };

    let mut player = commands.spawn((
        Sprite::from_image(image),
        SpritesheetAnimation::new(run_down_anim_handle),
        Transform::from_translation(Vec3::Z * 10.0) * Transform::from_scale(Vec3::splat(1.0)),
    ));

    player.insert(Player {
        speed: MOVE_SPEED,
        size: PLAYER_TILE_SIZE,
    });
}

fn move_player(
    mut q: Query<&mut Transform, With<Player>>,
    mut valid_move: MessageReader<MoveLegal>,
) {
    for event in valid_move.read() {
        if event.destination.is_none() {
            return;
        }
        if event.legal_move {
            //info!("Moving player to {:?}", event.destination);
            for mut transform in q.iter_mut() {
                transform.translation = Vec3::new(
                    event.destination.unwrap().x,
                    event.destination.unwrap().y,
                    10.0,
                );
            }
        }
    }
}

fn zoom_map(
    mut query_camera: Query<&mut Projection, With<PlayerCamera>>,
    mut scroll_evr: MessageReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    match game_state.get() {
        GameState::DirtyMap => {
            return; // Do not zoom when the map is dirty
        }
        _ => {}
    }
    let mut projection = query_camera.single_mut().unwrap();
    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        for ev in scroll_evr.read() {
            if ev.unit == MouseScrollUnit::Line {
                if ev.y == -1.0 {
                    // zoom in
                    projection2d.scale *= 1.25;
                } else if ev.y == 1.0 {
                    // zoom out
                    projection2d.scale /= 1.25;
                }
            }
        }
        if keyboard.just_pressed(KeyCode::KeyQ) {
            // zoom in
            projection2d.scale *= 1.25;
        } else if keyboard.just_pressed(KeyCode::KeyE) {
            // zoom out
            projection2d.scale /= 1.25;
        }
    }
}
