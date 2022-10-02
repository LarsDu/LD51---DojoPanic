use std::f32::consts::PI;

use bevy::{input::keyboard::*, prelude::*, utils::HashMap};

use super::animation::*;
use super::components::*;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_hero)
            .add_system(init_animation)
            .add_system(handle_input)
            .add_system(set_animation.after(handle_input))
            .add_system(set_rotation.after(handle_input));
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

    let mut hero_animation_map = HashMap::<BasicAnimationState, usize>::from([
        (BasicAnimationState::StandForward, 5),
        (BasicAnimationState::StandUp, 1),
        (BasicAnimationState::StandAngled, 0),
        (BasicAnimationState::WalkForward, 2),
        (BasicAnimationState::WalkUp, 4),
        (BasicAnimationState::WalkAngled, 3),
    ]);

    // Load the model
    let dojoman = asset_server.load("models/animated/dojoman.glb#Scene0");
    commands
        .spawn(SceneBundle {
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
            has_state_changed: true,
            look: Vec2::new(0.0, 0.0),
        })
        .insert(BasicAnimation {
            play_speed: 2.0,
            state: BasicAnimationState::StandForward,
            index_map: hero_animation_map,
        });
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Movement, With<Hero>>,
) {
    let mut movement =
        query.get_single_mut().expect("Error! Couldn't find hero");

    let prev_look = movement.look.clone();

    if keyboard_input.just_pressed(KeyCode::Left) {
        movement.look.x = -1.0;
    } else if keyboard_input.just_released(KeyCode::Left) {
        movement.look.x = 0.0;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        movement.look.x = 1.0;
    } else if keyboard_input.just_released(KeyCode::Right) {
        movement.look.x = 0.0;
    }

    // UP
    if keyboard_input.just_pressed(KeyCode::Up) {
        movement.look.y = 1.0;
    } else if keyboard_input.just_released(KeyCode::Up) {
        movement.look.y = 0.0;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
    } else if keyboard_input.just_released(KeyCode::Space) {
    }
    
    movement.has_state_changed = prev_look != movement.look;
    
}

fn set_animation(
    animations: Res<Animations>,
    mut query: Query<(&mut Movement, &BasicAnimation), With<Hero>>,
    mut animation_player: Query<&mut AnimationPlayer>,
) {
    if let Ok(mut animation_player) = animation_player.get_single_mut() {
        for (movement, basic_animation) in &mut query {
            if !movement.has_state_changed {
                continue;
            }

            let mut state = BasicAnimationState::StandForward;

            if movement.look.x.abs() > 0.0 && movement.look.y == 0.0 {
                // Play Walk animation
                state = BasicAnimationState::WalkForward;
            } else if movement.look.x == 0.0 && movement.look.y.abs() > 0.0{
                state = BasicAnimationState::StandUp;
            } else if movement.look.x.abs() > 0.0 && movement.look.y.abs() > 0.0{
                state = BasicAnimationState::WalkAngled;
            }

            // Play selected animation
            if let Some(i) = basic_animation
                .index_map
                .get(&state)
            {
                //println!("{}",animation_player.speed());
                //let speed = animation_player.speed();
                //animation_player.set_speed( speed * 1.2);
                animation_player.play(animations.0[*i].clone_weak()).repeat();
            }
        }
    }
}

fn set_rotation(mut query: Query<(&mut Movement, &mut Transform), With<Hero>>) {
    for (movement, mut transform) in &mut query {
        if !movement.has_state_changed {
            continue;
        }

        if movement.look.x > 0.0 {
            transform.look_at(Vec3::new(10000.0, 0.0, 0.0), Vec3::Y);
        } else if movement.look.x < 0.0 {
            transform.look_at(Vec3::new(-10000.0, 0.0, 0.0), Vec3::Y);
        }
    }
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
