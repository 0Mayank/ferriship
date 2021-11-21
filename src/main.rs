#![allow(unused)] // silince while learning

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1. / 60.;

// region: Resources
pub struct Materials {
    player: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
}

pub struct WinSize {
    h: f32,
    w: f32,
}
// endregion: Resources

// region: Components
struct Player;
struct PlayerReadyToFire(bool);
struct Laser;
struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}
// endregion: Components

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
        .add_system(player_movement.system())
        .add_system(player_fire.system())
        .add_system(laser_movement.system())
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
        player: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser: materials.add(asset_server.load(LASER_SPRITE).into()),
    });

    commands.insert_resource(WinSize {
        h: window.height(),
        w: window.width(),
    })
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, window: Res<WinSize>) {
    // spawn a sprite
    let bottom = -window.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerReadyToFire(true))
        .insert(Speed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    if let Ok((speed, mut transform)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };

        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut query: Query<(&Transform, &mut PlayerReadyToFire), With<Player>>,
) {
    if let Ok((player_transform, mut fire)) = query.single_mut() {
        if fire.0 && keyboard_input.pressed(KeyCode::Space) {
            let x = player_transform.translation.x;
            let y = player_transform.translation.y;

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y + 15., 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(Speed::default());
            fire.0 = false;
        }

        if keyboard_input.just_released(KeyCode::Space) {
            fire.0 = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    window: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), With<Laser>>,
) {
    for (laser_entity, speed, mut laser_transform) in query.iter_mut() {
        let translation = &mut laser_transform.translation;
        translation.y += speed.0 * TIME_STEP;
        if translation.y > window.h {
            commands.entity(laser_entity).despawn();
        }
    }
}
