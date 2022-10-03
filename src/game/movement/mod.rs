use super::super::constants::*;
use super::animation::*;
use super::components::*;
use super::hero::*;
use bevy::{prelude::*, time::FixedTimestep};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(init_animation)
                .with_system(set_animation.after(handle_input))
                .with_system(set_rotation.after(handle_input)),
        );
    }
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

            let mut animation_index = basic_animation.clip_indices.idle;

            if movement.look.x.abs() > 0.0 && movement.look.y == 0.0 {
                // Play Walk animation
                animation_index = basic_animation.clip_indices.walk;
            } else if movement.look.x == 0.0 && movement.look.y.abs() > 0.0 {
                animation_index = basic_animation.clip_indices.look_up;
            } else if movement.look.x.abs() > 0.0 && movement.look.y.abs() > 0.0 {
                animation_index = basic_animation.clip_indices.walk_look_angled;
            }

            animation_player
                .play(animations.0[animation_index].clone_weak())
                .repeat();
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
