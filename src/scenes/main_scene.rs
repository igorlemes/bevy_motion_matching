use bevy::asset::Assets;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Local, Query, Res, ResMut};
use bevy::input::gamepad::{Gamepad, GamepadButton, GamepadButtonType};
use bevy::input::ButtonInput;
use bevy::math::primitives::{Capsule3d, Cuboid, Plane3d};
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial};
use bevy::render::camera::{OrthographicProjection, ScalingMode};
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::transform::components::Transform;

use bevy_third_person_camera::camera::{CameraGamepadSettings, Zoom};
use bevy_third_person_camera::controller::*;
use bevy_third_person_camera::*;
use std::f32::consts::PI;

use crate::components::camera::CameraController;

pub fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        ThirdPersonCamera {
            zoom: Zoom::new(5.0, 20.0),
            gamepad_settings: CameraGamepadSettings {
                zoom_in_button: GamepadButton::new(
                    Gamepad::new(0),
                    GamepadButtonType::RightTrigger2,
                ),
                zoom_out_button: GamepadButton::new(
                    Gamepad::new(0),
                    GamepadButtonType::LeftTrigger2,
                ),
                ..Default::default()
            },
            ..Default::default()
        },
        Camera3dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(30.0),
                // ScalingMode::WindowSize(30.0),
                near: -100.0,
                ..Default::default()
            }
            .into(),
            transform: Transform::from_translation(Vec3::new(10.0, 10.0, 10.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        CameraController,
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
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
        // Simulation Bone
        PbrBundle {
            mesh: meshes.add(Mesh::from(Capsule3d::new(1.0, 2.0))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        },
        // 
        ThirdPersonCameraTarget,
        ThirdPersonController {
            speed: 3.5,
            sprint_speed: 4.5,
            gamepad_settings: ControllerGamepadSettings {
                sprint: GamepadButton::new(Gamepad::new(0), GamepadButtonType::LeftThumb),
                ..Default::default()
            },
            ..Default::default()
        },
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
