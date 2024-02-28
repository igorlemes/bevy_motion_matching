use bevy::asset::Assets;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Local, Query, Res, ResMut};
use bevy::input::gamepad::{Gamepad, GamepadButton, GamepadButtonType};
use bevy::input::ButtonInput;
use bevy::math::primitives::{Capsule3d, Circle, Cuboid, Plane3d};
use bevy::math::primitives::Plane2d;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial};
use bevy::render::camera::{OrthographicProjection, ScalingMode};
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::render::view::window;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::transform::components::Transform;
use bevy::window::Window;

use crate::components::spring::ControllerTrigger;

pub fn setup_2d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 5.0 })),
        material:  materials.add(Color::hsl(1.0, 0.5, 0.5)),
        // Put at 80% of the screen width and 20% of the screen height
        transform: Transform::from_translation(Vec3::new(740.0 * 0.8, 0.0, 0.0)),
        ..Default::default()
    },
    ControllerTrigger
    ));
}
