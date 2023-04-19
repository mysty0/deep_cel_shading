use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3406029"]
pub struct TestMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub matcap: Option<Handle<Image>>,
}

impl Material for TestMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/test_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
