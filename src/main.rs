mod mod_fighter;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_sprite)
    .add_startup_system(spawn_camera)
    .add_system(looped)
    .run();    
}

pub fn looped() {
    // this will loop
    mod_fighter::mod_fighter::StructFighter::walk_forward()
}

pub fn spawn_sprite(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("gfx/II.png"),
            ..default()
            },
        )
    );
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>)
{
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}