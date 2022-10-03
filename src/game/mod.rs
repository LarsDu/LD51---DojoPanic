use bevy::prelude::*;

mod hero;
use hero::*;

mod movement;
use movement::*;

mod tilemap;
use tilemap::*;

mod enemy;
use enemy::*;

mod spawner;
use spawner::*;


mod components;

mod animation;
//use animation::*;

use super::constants::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_lights)
            .add_plugin(HeroPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(TilemapPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(SpawnerPlugin);
    }
}





pub fn setup_lights(mut commands: Commands) {
    // Point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            radius: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
