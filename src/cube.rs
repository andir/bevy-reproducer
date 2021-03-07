use bevy::prelude::*;

use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{AssetRenderResourcesNode, RenderGraph};
use bevy::render::renderer::RenderResources;
use bevy::render::shader::{Shader, ShaderStages};

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<CubeMaterial>()
            .add_startup_system(add_cube.system());
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "ad9438f7-5c4d-49c3-818a-dd5d9d80004d"]
struct CubeMaterial {
    pub color: Color,
}

fn add_cube(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CubeMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    asset_server
        .watch_for_changes()
        .expect("failed to watch for asset changes");

    let vertex_shader = asset_server.load::<Shader, _>("shader/cube.vert");
    let fragment_shader = asset_server.load::<Shader, _>("shader/cube.frag");
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: vertex_shader,
        fragment: Some(fragment_shader),
    }));

    render_graph.add_system_node(
        "magic_cube_material",
        AssetRenderResourcesNode::<CubeMaterial>::new(true),
    );

    render_graph
        .add_node_edge(
            "magic_cube_material",
            bevy::render::render_graph::base::node::MAIN_PASS,
        )
        .expect("The resource must be there?!");

    let material = materials.add(CubeMaterial {
        color: Color::rgb(0.5, 0.8, 0.5),
    });

    commands
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.5,
                subdivisions: 16,
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .with(material);
}
