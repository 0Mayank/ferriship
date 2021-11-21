#![allow(unused)] // silince while learning

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Ferriship".to_string(),
            width: 598.0,
            height: 678.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // spawn a sprite
    let bottom = -window.height() / 2.;
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        transform: Transform {
            translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
