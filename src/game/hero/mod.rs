use std::f32::consts::PI;

use bevy::{input::keyboard::*, prelude::*};

use super::animation::*;
use super::components::*;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_hero)
            .add_system(init_animation)
            .add_system(keyboard_state_transitioner)
            .add_system(set_animation_direction.after(keyboard_state_transitioner));
    }
}

fn keyboard_state_transitioner(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Movement, &mut Transform), With<Hero>>,
    //animations: Res<Animations>,
    //mut animation_player: Query<&mut AnimationPlayer>,
) {
    //if let Ok(mut player) = animation_player.get_single_mut() {
    // Insert and remove components based on keyboard input
    let (mut movement, mut transform) = query.get_single_mut().expect("Error! Couldn't find hero");

    if keyboard_input.just_pressed(KeyCode::Left) {
        transform.look_at(Vec3::new(-100.0, 0.0, 0.0), Vec3::Y);
        //player.play(animations.0[WALK_FORWARD].clone_weak()).repeat();
        movement.look.x = -1.0;
        //commands.entity(hero_entity).insert(Walk);
    } else if keyboard_input.just_released(KeyCode::Left) {
        //player.play(animations.0[STAND_FORWARD].clone_weak()).repeat();
        movement.look.x = 0.0;
        //commands.entity(hero_entity).insert(Stand);
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        transform.look_at(Vec3::new(100.0, 0.0, 0.0), Vec3::Y);
        if keyboard_input.pressed(KeyCode::Up) {
            //player.play(animations.0[WALK_ANGLED].clone_weak()).repeat();
        } else {
            //player.play(animations.0[WALK_FORWARD].clone_weak()).repeat();
        }
        movement.look.x = 1.0;
    } else if keyboard_input.just_released(KeyCode::Right) {
        //player.play(animations.0[STAND_FORWARD].clone_weak()).repeat();
        movement.look.x = 0.0;
    }

    // UP
    if keyboard_input.just_pressed(KeyCode::Up) {
        //player.play(animations.0[STAND_UP].clone_weak()).repeat();
    } else if keyboard_input.just_released(KeyCode::Up) {
        //player.play(animations.0[STAND_FORWARD].clone_weak()).repeat();
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement.look.y = 1.0;
    } else if keyboard_input.just_released(KeyCode::Space) {
        movement.look.y = 0.0;
    }
    //}
}

fn set_animation_direction(
    animations: Res<Animations>,
    mut query: Query<(&mut Movement, &mut Transform), With<Hero>>,
    mut animation_player: Query<&mut AnimationPlayer>,
) {
    if let Ok(mut animation_player) = animation_player.get_single_mut() {
        for (movement, mut transform) in &mut query {
            if movement.look.x.abs() > 0.0 {
                animation_player
                    .play(animations.0[WALK_FORWARD].clone_weak())
                    .repeat();
            } else {
                animation_player
                    .play(animations.0[STAND_FORWARD].clone_weak())
                    .repeat();
            }
            //if movement.look.x > 0.0 {
            //    transform.look_at(Vec3::new(100.0, 0.0, 0.0), Vec3::Y);
            //} else if movement.look.x < 0.0 {
            //    transform.look_at(Vec3::new(-100.0, 0.0, 0.0), Vec3::Y);
           // }
        }
    }
}

pub fn setup_hero(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Insert animations
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/dojoman.glb#Animation0"), // 0 - stand forward
        asset_server.load("models/animated/dojoman.glb#Animation1"), // 1 - stand up
        asset_server.load("models/animated/dojoman.glb#Animation2"), // 2 - stand angled
        asset_server.load("models/animated/dojoman.glb#Animation3"), // 3 - walk forward
        asset_server.load("models/animated/dojoman.glb#Animation4"), //4 - walk up
        asset_server.load("models/animated/dojoman.glb#Animation5"), //5 - walk angled
    ]));

    // Load the model
    let dojoman = asset_server.load("models/animated/dojoman.glb#Scene0");
    commands
        .spawn_bundle(SceneBundle {
            scene: dojoman,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.0,
                -PI * 0.5,
                0.0,
            )), //.with_scale(Vec3::splat(0.60)),
            ..default()
        })
        .insert(Hero)
        .insert(Movement {
            look: Vec2::new(0.0, 0.0),
        });
}

// Once the scene is loaded, start the animation
fn init_animation(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play(animations.0[*current_animation].clone_weak())
                .repeat();
        }
    }
}
