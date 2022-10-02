use bevy::prelude::*;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum BasicAnimationState{
    StandForward,
    StandUp,
    StandAngled,
    WalkForward,
    WalkUp,
    WalkAngled
}

