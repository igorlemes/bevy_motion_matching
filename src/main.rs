//! Plays animations from a skinned glTF.

use std::f32::consts::PI;

use bevy::{
    input::gamepad::{GamepadAxisChangedEvent, GamepadAxisType, Gamepad, GamepadAxis},
    pbr::CascadeShadowConfigBuilder,
    color::palettes::css::{ORANGE, SILVER, WHITE},
    prelude::*,
};

use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};


#[derive(Resource)]
struct GamepadOne(Gamepad);

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .insert_resource(GamepadOne(Gamepad::new(0))) // Adicione o gamepad 0 como recurso
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_camera_update_azimuth)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100.0, 100.0, 150.0)
            .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
        ..default()
    });

    // Ground Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(500., 500.)),
        material: materials.add(Color::from(SILVER)),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

}



fn orbit_camera_update_azimuth(
    axes: Res<Axis<GamepadAxis>>,
    gamepad: Option<Res<GamepadOne>>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    let mut transform = query.single_mut();
    let Some(&GamepadOne(gamepad)) = gamepad.as_deref() else {
        // no gamepad is connected
        return;
    };

    // The joysticks are represented using a separate axis for X and Y
    let axis_rx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::RightStickX
    };
    let axis_ry = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::RightStickY
    };

    if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
        // Combine X and Y into one vector
        let mut right_stick = Vec2::new(x * x.abs(), y * y.abs());
        if right_stick.length() > 1.0 {
            right_stick = right_stick.normalize();
        }

        println!("Right stick: {:?}", right_stick);
        // Example: check if the stick is pushed up
            // Rotate the camera around the origin
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(right_stick.x));
    }

}

fn orbit_camera_update_azimuth_events(
    mut axis_event_reader: EventReader<GamepadAxisChangedEvent>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    let mut transform = query.single_mut();
    for ev in axis_event_reader.read() {
        match ev.axis_type {
            GamepadAxisType::RightStickX => {
                println!(
                    "Axis {:?} on gamepad {:?} is now at {:?}",
                    ev.axis_type, ev.gamepad, ev.value
                );
                transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(ev.value));
            }
            GamepadAxisType::RightStickY => {
                println!(
                    "Axis {:?} on gamepad {:?} is now at {:?}",
                    ev.axis_type, ev.gamepad, ev.value
                );
                transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(ev.value));
                
            }
            _ => {
                // we don't care about other events here (connect/disconnect)
            }
        }
    }
}