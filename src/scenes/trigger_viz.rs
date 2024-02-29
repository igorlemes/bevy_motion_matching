use std::collections::VecDeque;

use bevy::asset::Assets;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, ResMut};
use bevy::gizmos::primitives::dim2::Line2dBuilder;
use bevy::math::primitives::{Circle, Line2d};
use bevy::math::Vec3;
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::ui::Val;
use bevy::window::Window;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::transform::components::Transform;
use bevy::gizmos::gizmos::Gizmos;

use crate::components::spring::{ControllerTrigger, Spring, PositionHistory, SpringLines};


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
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 6.0 })),
            material: materials.add(Color::hsl(1.0, 0.5, 0.5)),
            transform: Transform::from_translation(Vec3::new(820.0 * 0.8, 0.0, 0.0)),
            ..Default::default()
        },
        ControllerTrigger,
        Spring::new(100),
        PositionHistory::new(100),
    ));

    // Spawn circles in a line at y = 0
    for i in 0..101 {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: 3.0 })),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0)),
                transform: Transform::from_translation(Vec3::new(i as f32 * - 20.0 + 820.0, 0.0, 0.0)),
                ..Default::default()
            },
            PositionHistory::new(100),
        ));
    }
}
