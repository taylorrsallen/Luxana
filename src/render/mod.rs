use crate::*;

use bevy::{
    core_pipeline::core_3d::Transparent3d,
    ecs::{
        query::QueryItem,
        system::{lifetimeless::*, SystemParamItem},
    },
    pbr::{
        MeshPipeline, MeshPipelineKey, RenderMeshInstances, SetMeshBindGroup, SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::{GpuBufferInfo, MeshVertexBufferLayout},
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, RenderCommand, RenderCommandResult,
            RenderPhase, SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::RenderDevice,
        view::{ExtractedView, NoFrustumCulling},
        Render, RenderApp, RenderSet,
    }, asset::load_internal_asset,
};

use bytemuck::{Pod, Zeroable};

////////////////////////////////////////////////////////////////////////////////////////////////////
const INSTANCING_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(2312391283770133547);

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankRenderPlugin;
impl Plugin for TankRenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, INSTANCING_SHADER_HANDLE, "instancing.wgsl", Shader::from_wgsl);

        app.add_plugins(InstancedPipelinePlugin)
            .add_systems(PostUpdate, sys_update_instanced_objects);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Deref)]
pub struct InstanceMaterialData(pub Vec<InstanceData>);

impl ExtractComponent for InstanceMaterialData {
    type Query = &'static InstanceMaterialData;
    type Filter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self> {
        Some(InstanceMaterialData(item.0.clone()))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct InstancedObject;

fn sys_update_instanced_objects(
    mut material_query: Query<&mut InstanceMaterialData>,
    object_query: Query<&GlobalTransform, With<InstancedObject>>,
) {
    let Ok(mut material) = material_query.get_single_mut() else { return };
    let mut data = vec![];
    for transform in object_query.iter() {
        data.push(InstanceData {
            position: transform.translation(),
        });
    }

    material.0 = data;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct InstancedPipelinePlugin;
impl Plugin for InstancedPipelinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, InstancedDraw>()
            .init_resource::<SpecializedMeshPipelines<InstancedPipeline>>()
            .add_systems(Render, (
                instanced_queue.in_set(RenderSet::QueueMeshes),
                instanced_prepare_buffers.in_set(RenderSet::PrepareResources),
            ));
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp).init_resource::<InstancedPipeline>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct InstanceData {
    pub position: Vec3,
    // pub rotation: Vec3,
}

fn instanced_queue(
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
    material_meshes: Query<Entity, With<InstanceMaterialData>>,
    mut pipelines: ResMut<SpecializedMeshPipelines<InstancedPipeline>>,
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    render_mesh_instances: Res<RenderMeshInstances>,
    custom_pipeline: Res<InstancedPipeline>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<Mesh>>,
    msaa: Res<Msaa>,
) {
    let instanced_draw = transparent_3d_draw_functions.read().id::<InstancedDraw>();
    let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples());

    for (view, mut transparent_phase) in &mut views {
        let view_key = msaa_key | MeshPipelineKey::from_hdr(view.hdr);
        let rangefinder = view.rangefinder3d();
        for entity in &material_meshes {
            let Some(mesh_instance) = render_mesh_instances.get(&entity) else { continue };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else { continue };
            let key = view_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
            let pipeline = pipelines.specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout).unwrap();

            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function: instanced_draw,
                distance: rangefinder.distance_translation(&mesh_instance.transforms.transform.translation),
                batch_range: 0..1,
                dynamic_offset: None,
            });
        }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn instanced_prepare_buffers(
    mut commands: Commands,
    query: Query<(Entity, &InstanceMaterialData)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instance_data) in &query {
        let instance_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("Instance Data Buffer"),
                contents: bytemuck::cast_slice(instance_data.as_slice()),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            });

        commands.entity(entity).insert(InstanceBuffer { buffer: instance_buffer, length: instance_data.len() });
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource)]
pub struct InstancedPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

impl FromWorld for InstancedPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let mesh_pipeline = world.resource::<MeshPipeline>();

        InstancedPipeline { shader: INSTANCING_SHADER_HANDLE, mesh_pipeline: mesh_pipeline.clone() }
    }
}

impl SpecializedMeshPipeline for InstancedPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        // meshes typically live in bind group 2. because we are using bindgroup 1
        // we need to add MESH_BINDGROUP_1 shader def so that the bindings are correctly
        // linked in the shader
        descriptor.vertex.shader_defs.push("MESH_BINDGROUP_1".into());

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
            ],
        });

        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
type InstancedDraw = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMeshInstanced,
);

pub struct DrawMeshInstanced;
impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = (SRes<RenderAssets<Mesh>>, SRes<RenderMeshInstances>);
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<InstanceBuffer>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        instance_buffer: &'w InstanceBuffer,
        (meshes, render_mesh_instances): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(mesh_instance) = render_mesh_instances.get(&item.entity()) else { return RenderCommandResult::Failure; };
        let gpu_mesh = match meshes.into_inner().get(mesh_instance.mesh_asset_id) {
                Some(gpu_mesh) => gpu_mesh,
                None => return RenderCommandResult::Failure,
            };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed { buffer, index_format, count } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            },
            GpuBufferInfo::NonIndexed => {
                pass.draw(0..gpu_mesh.vertex_count, 0..instance_buffer.length as u32);
            },
        }
        
        RenderCommandResult::Success
    }
}