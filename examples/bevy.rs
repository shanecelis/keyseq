//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use keyseq::{bevy::pkey, Modifiers};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

#[rustfmt::skip]
/// This system detects some key presses.
fn keyboard_input_system(input: Res<ButtonInput<KeyCode>>) {
    let mods = Modifiers::from(&input);
    for key in input.get_just_pressed() {
        match (mods, key) {
            pkey!{ Ctrl-A } | pkey!{ Super-A } => println!("Just pressed Ctrl-A or Super-A!"),
            pkey!{ Ctrl-Alt-A }                => println!("Just pressed Ctrl-Alt-A!"),
            pkey!{ Ctrl-Shift-A }              => println!("Just pressed Ctrl-Shift-A!"),
            pkey!{ Alt-Shift-A }               => println!("Just pressed Alt-Shift-A!"),
            pkey!{ Shift-A }                   => println!("Just pressed Shift-A!"),
            pkey!{ Alt-A }                     => println!("Just pressed Alt-A!"),
            pkey!{ Super-A }                   => println!("Just pressed Super-A!"),
            pkey!{ A }                         => println!("Just pressed A!"),
            _                                  => println!("No key matched"),
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
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(4.0))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    warn!("Press A key with different modifier keys.");
}
