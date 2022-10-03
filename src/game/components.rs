
use bevy::prelude::*;
use super::animation::*;
#[derive(Component)]
pub struct Hero;


// STATE COMPONENTS (no clean Finite state machine in ECS, yet)
#[derive(Component)]
pub struct Movement{
    pub has_state_changed: bool,
    pub look: Vec2,
}

#[derive(Component)]
pub struct BasicAnimation{
    pub clip_indices: AnimationClipIndices,
    pub play_speed: f32,
}

#[derive(Component)]
pub struct Air;

#[derive(Component)]
pub struct Grounded;


#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct Enemy;