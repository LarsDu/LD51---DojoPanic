use bevy::prelude::*;

pub struct Animations(pub Vec<Handle<AnimationClip>>);

pub const STAND_FORWARD: usize = 5;
pub const STAND_UP: usize = 1;
pub const STAND_ANGLED: usize = 0;
pub const WALK_FORWARD: usize = 2;
pub const WALK_UP: usize = 4;
pub const WALK_ANGLED: usize = 3;