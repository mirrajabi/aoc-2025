use bevy::{
    asset::{Asset, RenderAssetUsages},
    color::LinearRgba,
    math::Vec3,
    pbr::{Material, MaterialPipeline, MaterialPipelineKey},
    reflect::TypePath,
    render::{
        mesh::{Mesh, MeshVertexBufferLayoutRef},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

const SHADER_ASSET_PATH: &str = "shaders/line_material.wgsl";

#[derive(Debug, Clone)]
pub struct LineStrip {
    pub lines: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(value: LineStrip) -> Self {
        let vertices: Vec<_> = value.lines;
        Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> bevy::ecs::error::Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
