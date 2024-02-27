use bevy::asset::Assets;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::system::{Commands, ResMut};
use bevy::input::gamepad::{Gamepad, GamepadButton, GamepadButtonType};
use bevy::math::primitives::{Cuboid, Plane3d};
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial};
use bevy::render::camera::{OrthographicProjection, ScalingMode};
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::transform::components::Transform;
use bevy_third_person_camera::controller::*; // optional if you want movement controls
use bevy_third_person_camera::*;
use std::f32::consts::PI;

pub fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(30.0),
                near: -30.0,
                ..Default::default()
            }
            .into(),
            transform: Transform::from_translation(Vec3::new(10.0, 10.0, 10.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        ThirdPersonCamera::default(),
    ));
    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 0.0),
            rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4.0, PI / 4.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
        ThirdPersonCameraTarget,
        ThirdPersonController {
            speed: 3.5,
            sprint_speed: 4.5,
            gamepad_settings: ControllerGamepadSettings {
                sprint: GamepadButton::new(Gamepad::new(0), GamepadButtonType::LeftThumb), // default
            },
            ..Default::default()
        }, // optional if you want movement controls
    ));

    let plane_mesh = meshes.add(Plane3d::default());
    // Chessboard Plane
    let black_material = materials.add(StandardMaterial {
        base_color: Color::GRAY * 0.8,
        reflectance: 0.3,
        perceptual_roughness: 0.8,
        ..Default::default()
    });

    let white_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        reflectance: 1.0,
        perceptual_roughness: 0.8,
        ..Default::default()
    });

    let checkerboard_size = 32;
    for x in -(checkerboard_size - 1)..checkerboard_size {
        for z in -(checkerboard_size - 1)..checkerboard_size {
            commands.spawn((PbrBundle {
                mesh: plane_mesh.clone(),
                material: if (x + z) % 2 == 0 {
                    black_material.clone()
                } else {
                    white_material.clone()
                },
                transform: Transform::from_xyz(x as f32 * 2.0, -1.0, z as f32 * 2.0),
                ..Default::default()
            },));
        }
    }
}
