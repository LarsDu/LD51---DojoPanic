use std::f32::consts::PI;

use bevy::{input::keyboard::*, prelude::*, time::FixedTimestep};

use super::animation::*;
use super::components::*;
use super::super::constants::*;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_hero)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(handle_input)
        );
    }
}

fn setup_hero(
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
            has_state_changed: true,
            look: Vec2::new(0.0, 0.0),
        })
        .insert(BasicAnimation {
            clip_indices: AnimationClipIndices { idle: 5, look_up: 1, look_angled: 0, walk: 2, walk_look_up: 4, walk_look_angled: 3 },
            play_speed: 2.0,
        });
}

pub fn handle_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Movement, With<Hero>>) {
    let mut movement = query.get_single_mut().expect("Error! Couldn't find hero");

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



