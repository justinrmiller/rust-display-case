//! Loads and renders a glTF file as a scene.

use bevy::{prelude::*, window::PresentMode};

// Define a component to designate a rotation speed to an entity.
#[derive(Component)]
struct Rotatable {
    speed: f32,
}

use std::f32::consts::PI;

const FULL_TURN: f32 = 2.0 * PI;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
	.insert_resource(WindowDescriptor {
            title: "glTF Viewer".to_string(),
            width: 750.,
            height: 1334.,
            present_mode: PresentMode::Fifo,
            ..default()
    })
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(rotate_cube)
    .add_system(animate_light_direction)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // const transformBundle = bevy::prelude::TransformBundle {
    //     local: Transform::from_translation(Vec3::ZERO)
    // };

    const TRANSFORM_BUNDLE: bevy::prelude::TransformBundle = 
        TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO));

    commands.spawn().with_children(|parent| {
        parent.spawn_scene(
            asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0")
        );
    }).insert(Rotatable { speed: -0.1 }).insert_bundle(TRANSFORM_BUNDLE);

    // commands.spawn_bundle(SceneBundle {
    //     scene: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0"),
    //     transform: Transform::from_translation(Vec3::ZERO),
    //     ..default()
    // }).insert(Rotatable { speed: -0.1 });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            0.0, //time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

// This system will rotate any entity in the scene with an assigned Rotatable around its z-axis.
fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in cubes.iter_mut() {
        // The speed is taken as a percentage of a full 360 degree turn.
        // The timers delta_seconds is used to smooth out the movement.
        let rotation_change = Quat::from_rotation_y(FULL_TURN * cube.speed * timer.delta_seconds());
        transform.rotate(rotation_change);
    }
}
