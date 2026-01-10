use bevy::{
    app::{Plugin, Startup},
    core_pipeline::core_3d::Camera3d,
    ecs::system::Commands,
    math::Vec3,
    pbr::PointLight,
    prelude::*,
    utils::default,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};

const BACKGROUND_COLOR: Color = Color::srgb(0.05, 0.05, 0.05);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NoCameraPlayerPlugin)
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(MovementSettings {
                sensitivity: 0.00012, // default: 0.00012
                speed: 100000.0,      // default: 12.0
            })
            .add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        FlyCam,
        Transform::from_xyz(50000.0, 50000.0, 50000.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            // shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
