use crate::material_properties_types;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError},
    },
};
use bevy::{
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_mod_fbx::ATTRIBUTE_NORMAL_MAP_UV;

#[derive(Debug, Clone, ShaderType)]
pub struct Direction {
    forward: Vec3,
    right: Vec3,
}

const FORWARD_VEC: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const RIGHT_VEC: Vec3 = Vec3::new(-1.0, 0.0, 0.0);

impl Default for Direction {
    fn default() -> Self {
        Self {
            forward: FORWARD_VEC,
            right: RIGHT_VEC,
        }
    }
}

impl From<Quat> for Direction {
    fn from(quat: Quat) -> Self {
        return Direction {
            forward: quat * FORWARD_VEC,
            right: quat * RIGHT_VEC,
        };
    }
}

#[derive(Debug, Clone, ShaderType)]
pub struct ShadowRamp {
    width: f32,
    day_mult_colors: [Color; 5],
    night_mult_colors: [Color; 5],
    transition_range1: f32,
    transition_range2: f32,
    transition_range3: f32,
    transition_range4: f32,
    transition_range5: f32,
    transition_softness1: f32,
    transition_softness2: f32,
    transition_softness3: f32,
    transition_softness4: f32,
    transition_softness5: f32,
}

impl Default for ShadowRamp {
    fn default() -> Self {
        let default_color = Color::rgba(0.9, 0.7, 0.75, 1.0);
        let default_colors = [
            default_color.clone(),
            default_color.clone(),
            default_color.clone(),
            default_color.clone(),
            default_color.clone(),
        ];

        const TRANSITION_RANGE: f32 = 0.01;
        const SOFTNESS: f32 = 0.5;

        Self {
            width: 1.0,
            day_mult_colors: default_colors.clone(),
            night_mult_colors: default_colors,
            transition_range1: TRANSITION_RANGE,
            transition_range2: TRANSITION_RANGE,
            transition_range3: TRANSITION_RANGE,
            transition_range4: TRANSITION_RANGE,
            transition_range5: TRANSITION_RANGE,
            transition_softness1: SOFTNESS,
            transition_softness2: SOFTNESS,
            transition_softness3: SOFTNESS,
            transition_softness4: SOFTNESS,
            transition_softness5: SOFTNESS,
        }
    }
}

#[derive(Debug, Clone, ShaderType, Default)]
pub struct MaterialGlobalSpecular {
    shininess1: f32,
    shininess2: f32,
    shininess3: f32,
    shininess4: f32,
    shininess5: f32,
    specular_multi1: f32,
    specular_multi2: f32,
    specular_multi3: f32,
    specular_multi4: f32,
    specular_multi5: f32,
}

#[derive(Debug, Clone, ShaderType, Default)]
pub struct CelMaterialProperties {
    head_direction: Direction,
    shadow_ramp_values: ShadowRamp,
    global_specular: MaterialGlobalSpecular,

    use_materials: Vec4,
    metal_map_light_color: Color,
    metal_map_dark_color: Color,
    metal_map_shadow_multi_color: Color,

    metal_map_sharp_layer_color: Color,
    metal_map_specular_color: Color,

    specular_color: Color,
    hit_color: Color,

    day_night_cycle: f32,

    use_shadow_ramp_texture: f32,
    light_area: f32,
    flip_light_map: f32,
    face_map_softness: f32,
    use_ligth_map_color_ao: f32,
    use_vertex_color_ao: f32,

    normal_map_scale: f32,
    use_normal_map: f32,
    use_back_space_uv: f32,

    use_metal_map: f32,
    metal_map_tile_scale: f32,
    metal_map_brightness: f32,

    metal_map_shininess: f32,
    metal_map_sharp_layer_offset: f32,

    metal_map_specular_atten_in_shadow: f32,
    metal_map_specular_scale: f32,

    use_fresnel: f32,
    hit_color_fresnel_power: f32,
    hit_color_scaler: f32,

    rim_light_type: f32,
    rim_light_intensity: f32,
    rim_light_thickness: f32,
}

impl Into<Color> for material_properties_types::Color {
    fn into(self) -> Color {
        Color::rgba(self.r, self.g, self.b, self.a)
    }
}

impl From<material_properties_types::MaterialPropertiesRoot> for CelMaterialProperties {
    fn from(properties: material_properties_types::MaterialPropertiesRoot) -> Self {
        let properties = properties.m_saved_properties;
        let floats = properties.m_floats;
        let colors = properties.m_colors;
        CelMaterialProperties {
            head_direction: Default::default(),
            day_night_cycle: 0.0,
            use_shadow_ramp_texture: floats.use_shadow_ramp,
            use_normal_map: floats.use_bump_map,
            use_back_space_uv: floats.use_back_face_uv2,
            use_ligth_map_color_ao: floats.use_light_map_color_ao,
            use_vertex_color_ao: floats.use_vertex_color_ao,
            use_materials: [
                floats.use_material2,
                floats.use_material3,
                floats.use_material4,
                floats.use_material5,
            ]
            .into(),
            //use_face: floats.use_face_map_new,
            flip_light_map: 0.0,

            light_area: floats.light_area,
            face_map_softness: floats.face_map_softness,
            normal_map_scale: floats.bump_scale,

            shadow_ramp_values: ShadowRamp {
                width: floats.shadow_ramp_width,
                day_mult_colors: [
                    colors.first_shadow_mult_color.into(),
                    colors.first_shadow_mult_color2.into(),
                    colors.first_shadow_mult_color3.into(),
                    colors.first_shadow_mult_color4.into(),
                    colors.first_shadow_mult_color5.into(),
                ],
                night_mult_colors: [
                    colors.cool_shadow_mult_color.into(),
                    colors.cool_shadow_mult_color2.into(),
                    colors.cool_shadow_mult_color3.into(),
                    colors.cool_shadow_mult_color4.into(),
                    colors.cool_shadow_mult_color5.into(),
                ],
                transition_range1: floats.shadow_transition_range,
                transition_range2: floats.shadow_transition_range2,
                transition_range3: floats.shadow_transition_range3,
                transition_range4: floats.shadow_transition_range4,
                transition_range5: floats.shadow_transition_range5,

                transition_softness1: floats.shadow_transition_softness,
                transition_softness2: floats.shadow_transition_softness2,
                transition_softness3: floats.shadow_transition_softness3,
                transition_softness4: floats.shadow_transition_softness4,
                transition_softness5: floats.shadow_transition_softness5,
            },

            use_metal_map: floats.metal_material,
            metal_map_tile_scale: floats.mtmap_tile_scale,
            metal_map_brightness: floats.mtmap_brightness,
            metal_map_light_color: colors.mtmap_light_color.into(),
            metal_map_dark_color: colors.mtmap_dark_color.into(),
            metal_map_shadow_multi_color: colors.mtshadow_multi_color.into(),
            metal_map_shininess: floats.mtshininess,
            metal_map_sharp_layer_offset: floats.mtsharp_layer_offset,
            metal_map_sharp_layer_color: colors.mtsharp_layer_color.into(),
            metal_map_specular_color: colors.mtspecular_color.into(),
            metal_map_specular_atten_in_shadow: floats.mtspecular_atten_in_shadow,
            metal_map_specular_scale: floats.mtspecular_scale,
            global_specular: MaterialGlobalSpecular {
                shininess1: floats.shininess,
                shininess2: floats.shininess2,
                shininess3: floats.shininess3,
                shininess4: floats.shininess4,
                shininess5: floats.shininess5,
                specular_multi1: floats.spec_multi,
                specular_multi2: floats.spec_multi2,
                specular_multi3: floats.spec_multi3,
                specular_multi4: floats.spec_multi4,
                specular_multi5: floats.spec_multi5,
            },
            specular_color: colors.specular_color.into(),
            hit_color: colors.hit_color.into(),
            use_fresnel: 1.0,
            hit_color_fresnel_power: floats.hit_color_fresnel_power,
            hit_color_scaler: floats.hit_color_scaler,
            rim_light_type: 1.0,
            rim_light_intensity: 1.0,
            rim_light_thickness: 1.0,
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3406028"]
#[bind_group_data(CelMaterialKey)]
pub struct CelMaterial {
    pub is_face: bool,
    pub diffuse_only: bool,
    #[texture(0)]
    #[sampler(1)]
    diffuse: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    face_light_map: Option<Handle<Image>>,
    #[texture(4)]
    #[sampler(5)]
    light_map: Option<Handle<Image>>,
    #[texture(6)]
    #[sampler(7)]
    shadow_ramp: Option<Handle<Image>>,
    #[texture(8)]
    #[sampler(9)]
    normal_map: Option<Handle<Image>>,
    #[texture(10)]
    #[sampler(11)]
    metal_map: Option<Handle<Image>>,

    #[uniform(12)]
    properties: CelMaterialProperties,
}

impl CelMaterial {
    pub fn new(
        diffuse: Handle<Image>,
        light_map: Handle<Image>,
        shadow_ramp: Handle<Image>,
        metal_map: Handle<Image>,
        normal_map: Option<Handle<Image>>,
        properties: CelMaterialProperties,
    ) -> Self {
        Self {
            is_face: false,
            diffuse_only: false,
            diffuse: Some(diffuse),
            light_map: Some(light_map),
            metal_map: Some(metal_map),
            shadow_ramp: Some(shadow_ramp),
            face_light_map: None,
            normal_map,
            properties,
        }
    }

    pub fn new_face(
        diffuse: Handle<Image>,
        face_light_map: Handle<Image>,
        light_map: Handle<Image>,
        metal_map: Handle<Image>,
        shadow_ramp: Option<Handle<Image>>,
        properties: CelMaterialProperties,
    ) -> Self {
        Self {
            is_face: true,
            diffuse_only: false,
            diffuse: Some(diffuse),
            face_light_map: Some(face_light_map),
            metal_map: Some(metal_map),
            light_map: Some(light_map),
            shadow_ramp: shadow_ramp,
            normal_map: None,
            properties,
        }
    }

    pub fn update_head_direction(&mut self, head_direction: Direction) {
        self.properties.head_direction = head_direction;
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CelMaterialKey {
    is_face: bool,
    diffuse_only: bool,
}

impl From<&CelMaterial> for CelMaterialKey {
    fn from(material: &CelMaterial) -> Self {
        CelMaterialKey {
            is_face: material.is_face,
            diffuse_only: material.diffuse_only,
        }
    }
}

impl Material for CelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cel_material.wgsl".into()
        //"shaders/test.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/cel_material.wgsl".into()
        //"shaders/test.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if !layout.contains(Mesh::ATTRIBUTE_TANGENT) || !layout.contains(Mesh::ATTRIBUTE_COLOR) {
            if let Some(fragment) = descriptor.fragment.as_mut() {
                fragment.shader_defs.push("EMPTY".into());
            }
            //descriptor.vertex.buffers = vec![layout.get_layout(&[])?];
            return Ok(());
        }

        println!("has color {}", layout.contains(Mesh::ATTRIBUTE_COLOR));

        let mut vertex_attributes = vec![
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_TANGENT.at_shader_location(2),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(3),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(4),
        ];

        let mut shader_defs = Vec::new();

        if key.bind_group_data.diffuse_only {
            shader_defs.push("SIMPLE".into());
        }

        if key.bind_group_data.is_face {
            shader_defs.push("FACE".into());
        } else {
            // if !layout.contains(ATTRIBUTE_NORMAL_MAP_UV) {

            // }
            if layout.contains(ATTRIBUTE_NORMAL_MAP_UV) {
                vertex_attributes.push(ATTRIBUTE_NORMAL_MAP_UV.at_shader_location(5));
                shader_defs.push("VERTEX_NORMAL_MAP_UV".into());
            }
            println!(
                "has normal map uv {}",
                layout.contains(ATTRIBUTE_NORMAL_MAP_UV)
            );
        }

        let vertex_layout = layout.get_layout(&vertex_attributes)?;
        descriptor.vertex.buffers = vec![vertex_layout];

        println!("defs {:?}", &shader_defs);
        descriptor.vertex.shader_defs.extend(shader_defs.clone());
        if let Some(fragment) = descriptor.fragment.as_mut() {
            fragment.shader_defs.extend(shader_defs);
        }

        descriptor.primitive.cull_mode = None;

        Ok(())
    }
}
