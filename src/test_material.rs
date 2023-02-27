use bevy::{render::{render_resource::{AsBindGroup, ShaderRef, OwnedBindingResource, encase::{self, UniformBuffer}, ShaderType}, Extract, RenderStage, RenderApp, renderer::RenderQueue, extract_resource::ExtractResource}, prelude::*, sprite::{Material2d, RenderMaterials2d}, reflect::TypeUuid, ui::update};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "e64f0929-1f3e-4ad8-8346-e4d4764eb34b"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub time: f32,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}

pub struct TestCustomMaterialPlugin;

impl Plugin for TestCustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Prepare, update_custom_mat)
        .add_system_to_stage(RenderStage::Extract, extract_custom_mat_entities_from_world_to_render);
    }
}

#[derive(Clone, ShaderType)]
struct CustomMaterialUniformData {
    time: f32
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/test.wgsl".into()
    }
}

fn update_custom_mat(
    materials: Res<RenderMaterials2d<CustomMaterial>>,
    entities_with_material: Query<&Handle<CustomMaterial>>,
    render_queue: Res<RenderQueue>,
    time: Res<Time>,
) {
    for handle in &entities_with_material {
        let Some(mat) = materials.get(handle) else {continue};
        for binding in mat.bindings.iter() {
            let OwnedBindingResource::Buffer(cur_buffer) = binding else {continue};
            let mut buffer: UniformBuffer<Vec<u8>> = encase::UniformBuffer::new(Vec::new());
            let mut rng = rand::thread_rng();
            buffer.write(&CustomMaterialUniformData {
                time: time.elapsed_seconds_wrapped()
            }).unwrap();
            render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
        }
    }
}

fn extract_custom_mat_entities_from_world_to_render(
    mut commands: Commands,
    entities_with_material: Extract<Query<(Entity, &Handle<CustomMaterial>)>>,
) {
    for (ent, handle) in &entities_with_material {
        commands.get_or_spawn(ent)
        .insert(handle.clone());
    }
}