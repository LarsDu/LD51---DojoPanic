use bevy::prelude::*;

#[derive(Component)]
pub struct Hero;


// STATE COMPONENTS (no clean Finite state machine in ECS, yet)
#[derive(Component)]
pub struct Movement{
    pub look: Vec2,
}


#[derive(Component)]
pub struct Jump;

#[derive(Component)]
pub struct Walk;

#[derive(Component)]
pub struct Stand;

#[derive(Component)]
pub struct Angled45;

#[derive(Component)]
pub struct Firing;