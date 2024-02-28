use bevy::asset::Assets;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::system::{Commands, ResMut};
use bevy::math::primitives::Circle;
use bevy::math::Vec3;
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::transform::components::Transform;

use crate::components::spring::{ControllerTrigger, Spring};

pub fn setup_2d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 8.0 })),
            material: materials.add(Color::hsl(1.0, 0.5, 0.5)),
            // Put at 80% of the screen width and 20% of the screen height
            transform: Transform::from_translation(Vec3::new(740.0 * 0.8, 0.0, 0.0)),
            ..Default::default()
        },
        ControllerTrigger,
        Spring::new(10),
    ));
}
