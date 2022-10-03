use bevy::prelude::*;

//#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

pub struct AnimationClipIndices{
    // Store the gltf for various states
    pub idle: usize,
    pub look_up: usize,
    pub look_angled: usize,
    pub walk: usize,
    pub walk_look_up: usize,
    pub walk_look_angled: usize,
}

