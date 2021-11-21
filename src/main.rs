#![allow(unused)] // silince while learning

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";

// region: Resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
}

pub struct WinSize {
    h: f32,
    w: f32,
}
// endregion: Resources

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
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
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

    // create the main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
    });

    commands.insert_resource(WinSize {
        h: window.height(),
        w: window.width(),
    })
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, window: Res<WinSize>) {
    // spawn a sprite
    let bottom = -window.h / 2.;
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_materials.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
