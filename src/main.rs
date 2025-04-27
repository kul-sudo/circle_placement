//! Illustrates different lights of various types and colors, some static, some moving over
//! a simple scene.

use bevy::{
    color::palettes::css::*,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
    color::palettes::css::*,
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_3;
use std::f32::consts::PI;
use std::ops::Range;

const ORBIT_RADIUS: f32 = 10.0;
const ALPHA_DELTA: f32 = 0.001;

const CUBE_SIZE: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource::<CameraSettings>(CameraSettings::default())
        .insert_resource(Parameters(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 125.0,
            sensitivity_iso: 100.0,
            sensor_height: 0.01866,
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_light_direction, orbit))
        .run();
}

#[derive(Debug, Resource)]
struct CameraSettings {
    alpha: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self { alpha: 0.0 }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

#[derive(Component)]
struct Movable;

/// set up a simple 3D scene
fn setup(
    parameters: Res<Parameters>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LIMEGREEN.into(),
            ..default()
        })),
        Transform::from_xyz(0.1, 0.1, 1.5),
        Movable,
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Wireframe,
        // This lets you configure the wireframe color of this entity.
        // If not set, this will use the color in `WireframeConfig`
        WireframeColor { color: LIME.into() },
    ));

    // sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LIMEGREEN.into(),
            ..default()
        })),
        Transform::from_xyz(1.5, 1.0, 1.5),
        Movable,
    ));

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder::default().build(),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 5.0).looking_at(Vec3::ZERO, Vec3::ZERO),
        Exposure::from_physical_camera(**parameters),
        Movable,
    ));
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.1);
    }
}

fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
    time: Res<Time>,
) {
    // let delta = vec2(0.1, 0.0);
    //
    // let delta_pitch = delta.y * camera_settings.pitch_speed;
    // let delta_yaw = delta.x * camera_settings.yaw_speed;
    //
    // // Obtain the existing pitch, yaw, and roll values from the transform.
    // let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    //
    // // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    // let pitch = (pitch + delta_pitch).clamp(
    //     camera_settings.pitch_range.start,
    //     camera_settings.pitch_range.end,
    // );
    // let yaw = yaw + delta_yaw;
    // camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    //
    // // Adjust the translation to maintain the correct orientation toward the orbit target.
    // // In our example it's a static target, but this could easily be customized.
    // let target = Vec3::ZERO;
    camera_settings.alpha += ALPHA_DELTA;
    camera.translation.x = ORBIT_RADIUS * camera_settings.alpha.cos();
    camera.translation.z = ORBIT_RADIUS * camera_settings.alpha.sin();
    camera.look_at(Vec3::ZERO, Vec3::ZERO);
    // camera.translation = target - camera.forward() * camera_settings.orbit_distance;
}
