//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*, render::mesh::shape::Circle};
use keyseq::{Modifiers, bevy::{pkey, pkeyseq}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

/// This system prints when `Ctrl + Shift + A` is pressed
fn keyboard_input_system(input: Res<Input<KeyCode>>) {
    let mods = Modifiers::from_input(&input);


    let my_mods = Modifiers::SHIFT | Modifiers::CONTROL;
    for key in input.get_just_pressed() {
        // info!("{} {:?}", mods.bits(), key);
        info!("{:?} vs {:?}", (mods, key), pkey!(ctrl-shift-A));
        // info!("{}", pkey!(ctrl-shift-A).0.bits());
        match (mods, key) {
            // pkey!(A) => info!("Just pressed A!"),
            pkey!(ctrl-A) => info!("Just pressed ctrl-A!"),
            pkey!(ctrl-alt-A) => info!("Just pressed ctrl-alt-A!"),
            // pkey!(ctrl-shift-A) => info!("Just pressed ctrl-shift-A!"),
            pkey!(alt-shift-A) => info!("Just pressed alt-shift-A!"),
            pkey!(shift-A) => info!("Just pressed shift-A!"),
            (my_mods, KeyCode::A)  => info!("Just pressed ctrl-shift-A!"),
            // (Modifiers::SHIFT | Modifiers::CONTROL, KeyCode::A)  => info!("Just pressed ctrl-shift-A!"),
            _ => info!("No key matched"),

        }
        if key == &KeyCode::A {
            info!("matched");
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
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
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
