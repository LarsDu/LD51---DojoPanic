use bevy::{prelude::*, render::camera::*};

mod menu;
use menu::MenuPlugin;
use bevy_rapier2d::prelude::*;
mod game;
use game::GamePlugin;


mod constants;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Menu,
}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Dojo Panic!".to_string(),
            width: 1024.0,
            height: 800.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state(AppState::Menu)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(setup_camera)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    /*commands.spawn_bundle(Camera2dBundle{
        ..default()
    }
    );*/
    // Bevy 2d camera is at Z=999.9
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 160.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

