use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    ecs::system::Commands,
    math::Vec3,
    pbr::DirectionalLight,
    transform::components::Transform,
    utils::default,
};
use d8_rs::{camera::CameraPlugin, diagnostics::DiagnosticsPlugin, viz::VizPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, DiagnosticsPlugin))
        .add_plugins(VizPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        Transform::IDENTITY.looking_at(Vec3::ONE, Vec3::NEG_ONE),
    ));
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        Transform::IDENTITY.looking_at(Vec3::NEG_ONE, Vec3::ONE),
    ));
}
