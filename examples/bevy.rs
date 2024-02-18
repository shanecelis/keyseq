//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*, input::ButtonInput as Input};
use keyseq::{bevy::pkey, Modifiers};

fn main() {
    println!("Press A key with different modifier keys.");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

#[rustfmt::skip]
/// This system detects some key presses.
fn keyboard_input_system(input: Res<Input<KeyCode>>) {
    let mods = Modifiers::from_input(&input);
    for key in input.get_just_pressed() {
        match (mods, key) {
            pkey!(ctrl-A) | pkey!(super-A) => println!("Just pressed ctrl-A or super-A!"),
            pkey!(ctrl-alt-A)              => println!("Just pressed ctrl-alt-A!"),
            pkey!(ctrl-shift-A)            => println!("Just pressed ctrl-shift-A!"),
            pkey!(alt-shift-A)             => println!("Just pressed alt-shift-A!"),
            pkey!(shift-A)                 => println!("Just pressed shift-A!"),
            pkey!(alt-A)                   => println!("Just pressed alt-A!"),
            pkey!(super-A)                 => println!("Just pressed super-A!"),
            pkey!(A)                       => println!("Just pressed A!"),
            _                              => println!("No key matched"),
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::default())),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
