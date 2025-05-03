use bevy::{
    color::palettes::css::*,
    pbr::{CascadeShadowConfigBuilder, wireframe::{Wireframe, WireframeColor}},
    prelude::*,
};
use std::f32::consts::PI;

const ORBIT_RADIUS: f32 = 10.0;
const ALPHA_DELTA: f32 = 0.001;

const CUBE_SIZE: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource::<CameraSettings>(CameraSettings::default())
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

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LIMEGREEN.into(),
            ..default()
        })),
        Transform::from_xyz(0.1, 0.1, 1.5),
    ));

    // sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LIMEGREEN.into(),
            ..default()
        })),
        Transform::from_xyz(-5.0, 0.1, 1.0),
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
        Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LIMEGREEN.into(),
            ..default()
        })),
        Transform::from_xyz(1.5, 1.0, 1.5),
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
        DistanceFog {
            color: Color::srgb(0.25, 0.25, 0.25),
            falloff: FogFalloff::ExponentialSquared { density: 0.05 },
            ..default()
        },
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
) {
    camera_settings.alpha += ALPHA_DELTA;
    camera.translation.x = ORBIT_RADIUS * camera_settings.alpha.cos();
    camera.translation.z = ORBIT_RADIUS * camera_settings.alpha.sin();
    camera.look_at(Vec3::ZERO, Vec3::Y);
}
