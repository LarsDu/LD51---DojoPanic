
use bevy::{prelude::*, utils::HashMap};
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
    pub state: BasicAnimationState,
    pub play_speed: f32,
    pub index_map: HashMap<BasicAnimationState, usize>


}

#[derive(Component)]
pub struct Jump;

#[derive(Component)]
pub struct Walk;

#[derive(Component)]
pub struct Stand;

#[derive(Component)]
pub struct Weapon;