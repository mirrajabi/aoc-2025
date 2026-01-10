use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::Assets,
    color::{Color, LinearRgba},
    ecs::system::{Commands, ResMut},
    math::{Vec3, primitives::Cuboid},
    pbr::{MaterialPlugin, MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d},
    tasks::{AsyncComputeTaskPool, futures_lite::future},
    transform::components::Transform,
    utils::default,
};

use crate::{
    circuits::{ComputeTask, calculate},
    mesh::{LineMaterial, LineStrip},
};

pub struct VizPlugin;

impl Plugin for VizPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<LineMaterial>::default())
            .add_systems(Startup, start_calculation)
            .add_systems(Update, poll_calculation);
    }
}

fn start_calculation(mut commands: Commands) {
    let pool = AsyncComputeTaskPool::get();
    let task = pool.spawn(async { calculate("./assets/personal.txt", Some(10)) });
    commands.insert_resource(ComputeTask(task));
}

fn poll_calculation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    task: Option<ResMut<ComputeTask>>,
) {
    let Some(mut task) = task else { return };

    let poll = future::block_on(future::poll_once(&mut task.0));
    let Some((_, points, circuits)) = poll else {
        return;
    };

    commands.remove_resource::<ComputeTask>();
    circuits.iter().cloned().for_each(|c| {
        commands.spawn((
            Mesh3d(meshes.add(LineStrip { lines: c.jboxes })),
            MeshMaterial3d(line_materials.add(LineMaterial { color: c.color })),
            Transform::IDENTITY,
        ));
    });

    let cube_size = 100000;
    let cube_linestrip: Vec<Vec3> = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(cube_size as f32, 0.0, 0.0),
        Vec3::new(cube_size as f32, cube_size as f32, 0.0),
        Vec3::new(0.0, cube_size as f32, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, cube_size as f32),
        Vec3::new(cube_size as f32, 0.0, cube_size as f32),
        Vec3::new(cube_size as f32, cube_size as f32, cube_size as f32),
        Vec3::new(0.0, cube_size as f32, cube_size as f32),
        Vec3::new(0.0, 0.0, cube_size as f32),
        Vec3::new(cube_size as f32, 0.0, cube_size as f32),
        Vec3::new(cube_size as f32, 0.0, 0.0),
        Vec3::new(cube_size as f32, cube_size as f32, 0.0),
        Vec3::new(cube_size as f32, cube_size as f32, cube_size as f32),
        Vec3::new(0.0, cube_size as f32, cube_size as f32),
        Vec3::new(0.0, cube_size as f32, 0.0),
    ];
    commands.spawn((
        Mesh3d(meshes.add(LineStrip {
            lines: cube_linestrip,
        })),
        MeshMaterial3d(line_materials.add(LineMaterial {
            color: LinearRgba::WHITE,
        })),
        Transform::IDENTITY,
    ));

    let point_size = 100.0f32;
    let point_color = Color::srgb(0.8f32, 0.3f32, 0.2f32);
    for point in points {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(point_size, point_size, point_size))),
            MeshMaterial3d(std_materials.add(StandardMaterial {
                base_color: point_color,
                ..default()
            })),
            Transform::from_xyz(point.0 as f32, point.1 as f32, point.2 as f32),
        ));
    }
}
