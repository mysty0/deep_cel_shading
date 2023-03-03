use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct MaterialPropertiesRoot {
    #[serde(rename = "m_Shader")]
    pub m_shader: MShader,
    #[serde(rename = "m_SavedProperties")]
    pub m_saved_properties: MSavedProperties,
    #[serde(rename = "m_StringTagMap")]
    pub m_string_tag_map: MStringTagMap,
    #[serde(rename = "m_Name")]
    pub m_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MShader {
    #[serde(rename = "m_FileID")]
    pub m_file_id: i64,
    #[serde(rename = "m_PathID")]
    pub m_path_id: i64,
    #[serde(rename = "IsNull")]
    pub is_null: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MSavedProperties {
    #[serde(rename = "m_TexEnvs")]
    pub m_tex_envs: MTexEnvs,
    #[serde(rename = "m_Ints")]
    pub m_ints: Value,
    #[serde(rename = "m_Floats")]
    pub m_floats: MFloats,
    #[serde(rename = "m_Colors")]
    pub m_colors: MColors,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MTexEnvs {
    #[serde(rename = "_AnimTextureQ")]
    pub anim_texture_q: Texture,
    #[serde(rename = "_AnimTextureT")]
    pub anim_texture_t: Texture,
    #[serde(rename = "_BumpMap")]
    pub bump_map: Texture,
    #[serde(rename = "_ClipAlphaTex")]
    pub clip_alpha_tex: Texture,
    #[serde(rename = "_FaceMapTex")]
    pub face_map_tex: Texture,
    #[serde(rename = "_LightMapTex")]
    pub light_map_tex: Texture,
    #[serde(rename = "_MTMap")]
    pub mtmap: Texture,
    #[serde(rename = "_MTSpecularRamp")]
    pub mtspecular_ramp: Texture,
    #[serde(rename = "_MainTex")]
    pub main_tex: Texture,
    #[serde(rename = "_MaterialMasksTex")]
    pub material_masks_tex: Texture,
    #[serde(rename = "_PackedShadowRampTex")]
    pub packed_shadow_ramp_tex: Texture,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    #[serde(rename = "m_Texture")]
    pub m_texture: TextureFile,
    #[serde(rename = "m_Scale")]
    pub m_scale: Vec2,
    #[serde(rename = "m_Offset")]
    pub m_offset: Vec2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureFile {
    #[serde(rename = "m_FileID")]
    pub m_file_id: i64,
    #[serde(rename = "m_PathID")]
    pub m_path_id: i64,
    #[serde(rename = "IsNull")]
    pub is_null: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vec2 {
    #[serde(rename = "X")]
    pub x: f32,
    #[serde(rename = "Y")]
    pub y: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MFloats {
    #[serde(rename = "_AnimBoneOffset")]
    pub anim_bone_offset: f32,
    #[serde(rename = "_AnimFPS")]
    pub anim_fps: f32,
    #[serde(rename = "_BumpScale")]
    pub bump_scale: f32,
    #[serde(rename = "_CharacterAmbientSensorColorOn")]
    pub character_ambient_sensor_color_on: f32,
    #[serde(rename = "_CharacterAmbientSensorForceShadowOn")]
    pub character_ambient_sensor_force_shadow_on: f32,
    #[serde(rename = "_CharacterAmbientSensorShadowOn")]
    pub character_ambient_sensor_shadow_on: f32,
    #[serde(rename = "_ClipAlphaHighLightScale")]
    pub clip_alpha_high_light_scale: f32,
    #[serde(rename = "_ClipAlphaThreshold")]
    pub clip_alpha_threshold: f32,
    #[serde(rename = "_ClipAlphaUVSet")]
    pub clip_alpha_uvset: f32,
    #[serde(rename = "_ClipBoxHighLightScale")]
    pub clip_box_high_light_scale: f32,
    #[serde(rename = "_ClipDissolveDirection")]
    pub clip_dissolve_direction: f32,
    #[serde(rename = "_ClipDissolveHightlightScale")]
    pub clip_dissolve_hightlight_scale: f32,
    #[serde(rename = "_ClipDissolveValue")]
    pub clip_dissolve_value: f32,
    #[serde(rename = "_ClipMethod")]
    pub clip_method: f32,
    #[serde(rename = "_ClipPlaneWorld")]
    pub clip_plane_world: f32,
    #[serde(rename = "_CullMode")]
    pub cull_mode: f32,
    #[serde(rename = "_Cutoff")]
    pub cutoff: f32,
    #[serde(rename = "_DetailNormalMapScale")]
    pub detail_normal_map_scale: f32,
    #[serde(rename = "_DitherAlpha")]
    pub dither_alpha: f32,
    #[serde(rename = "_DrawBackFace")]
    pub draw_back_face: f32,
    #[serde(rename = "_DstBlend")]
    pub dst_blend: f32,
    #[serde(rename = "_ElementViewEleID")]
    pub element_view_ele_id: f32,
    #[serde(rename = "_EmissionScaler")]
    pub emission_scaler: f32,
    #[serde(rename = "_EmissionScaler1")]
    pub emission_scaler1: f32,
    #[serde(rename = "_EmissionScaler2")]
    pub emission_scaler2: f32,
    #[serde(rename = "_EmissionScaler3")]
    pub emission_scaler3: f32,
    #[serde(rename = "_EmissionScaler4")]
    pub emission_scaler4: f32,
    #[serde(rename = "_EmissionScaler5")]
    pub emission_scaler5: f32,
    #[serde(rename = "_EmissionStrengthLerp")]
    pub emission_strength_lerp: f32,
    #[serde(rename = "_FaceBlushStrength")]
    pub face_blush_strength: f32,
    #[serde(rename = "_FaceMapRotateOffset")]
    pub face_map_rotate_offset: f32,
    #[serde(rename = "_FaceMapSoftness")]
    pub face_map_softness: f32,
    #[serde(rename = "_GlossMapScale")]
    pub gloss_map_scale: f32,
    #[serde(rename = "_Glossiness")]
    pub glossiness: f32,
    #[serde(rename = "_GlossyReflections")]
    pub glossy_reflections: f32,
    #[serde(rename = "_HitColorFresnelPower")]
    pub hit_color_fresnel_power: f32,
    #[serde(rename = "_HitColorScaler")]
    pub hit_color_scaler: f32,
    #[serde(rename = "_InstanceData")]
    pub instance_data: f32,
    #[serde(rename = "_LightArea")]
    pub light_area: f32,
    #[serde(rename = "_MTMapBrightness")]
    pub mtmap_brightness: f32,
    #[serde(rename = "_MTMapTileScale")]
    pub mtmap_tile_scale: f32,
    #[serde(rename = "_MTSharpLayerOffset")]
    pub mtsharp_layer_offset: f32,
    #[serde(rename = "_MTShininess")]
    pub mtshininess: f32,
    #[serde(rename = "_MTSpecularAttenInShadow")]
    pub mtspecular_atten_in_shadow: f32,
    #[serde(rename = "_MTSpecularScale")]
    pub mtspecular_scale: f32,
    #[serde(rename = "_MTUseSpecularRamp")]
    pub mtuse_specular_ramp: f32,
    #[serde(rename = "_MainTexAlphaCutoff")]
    pub main_tex_alpha_cutoff: f32,
    #[serde(rename = "_MainTexAlphaUse")]
    pub main_tex_alpha_use: f32,
    #[serde(rename = "_MainTexColoring")]
    pub main_tex_coloring: f32,
    #[serde(rename = "_MaxOutlineZOffset")]
    pub max_outline_zoffset: f32,
    #[serde(rename = "_MetalMaterial")]
    pub metal_material: f32,
    #[serde(rename = "_Metallic")]
    pub metallic: f32,
    #[serde(rename = "_Mode")]
    pub mode: f32,
    #[serde(rename = "_OcclusionStrength")]
    pub occlusion_strength: f32,
    #[serde(rename = "_OutlinePolygonOffsetFactor")]
    pub outline_polygon_offset_factor: f32,
    #[serde(rename = "_OutlinePolygonOffsetUnit")]
    pub outline_polygon_offset_unit: f32,
    #[serde(rename = "_OutlineType")]
    pub outline_type: f32,
    #[serde(rename = "_OutlineWidth")]
    pub outline_width: f32,
    #[serde(rename = "_Parallax")]
    pub parallax: f32,
    #[serde(rename = "_PolygonOffsetFactor")]
    pub polygon_offset_factor: f32,
    #[serde(rename = "_PolygonOffsetUnit")]
    pub polygon_offset_unit: f32,
    #[serde(rename = "_Scale")]
    pub scale: f32,
    #[serde(rename = "_ShadowRampWidth")]
    pub shadow_ramp_width: f32,
    #[serde(rename = "_ShadowTransitionRange")]
    pub shadow_transition_range: f32,
    #[serde(rename = "_ShadowTransitionRange2")]
    pub shadow_transition_range2: f32,
    #[serde(rename = "_ShadowTransitionRange3")]
    pub shadow_transition_range3: f32,
    #[serde(rename = "_ShadowTransitionRange4")]
    pub shadow_transition_range4: f32,
    #[serde(rename = "_ShadowTransitionRange5")]
    pub shadow_transition_range5: f32,
    #[serde(rename = "_ShadowTransitionSoftness")]
    pub shadow_transition_softness: f32,
    #[serde(rename = "_ShadowTransitionSoftness2")]
    pub shadow_transition_softness2: f32,
    #[serde(rename = "_ShadowTransitionSoftness3")]
    pub shadow_transition_softness3: f32,
    #[serde(rename = "_ShadowTransitionSoftness4")]
    pub shadow_transition_softness4: f32,
    #[serde(rename = "_ShadowTransitionSoftness5")]
    pub shadow_transition_softness5: f32,
    #[serde(rename = "_Shininess")]
    pub shininess: f32,
    #[serde(rename = "_Shininess2")]
    pub shininess2: f32,
    #[serde(rename = "_Shininess3")]
    pub shininess3: f32,
    #[serde(rename = "_Shininess4")]
    pub shininess4: f32,
    #[serde(rename = "_Shininess5")]
    pub shininess5: f32,
    #[serde(rename = "_SmoothnessTextureChannel")]
    pub smoothness_texture_channel: f32,
    #[serde(rename = "_SpecMulti")]
    pub spec_multi: f32,
    #[serde(rename = "_SpecMulti2")]
    pub spec_multi2: f32,
    #[serde(rename = "_SpecMulti3")]
    pub spec_multi3: f32,
    #[serde(rename = "_SpecMulti4")]
    pub spec_multi4: f32,
    #[serde(rename = "_SpecMulti5")]
    pub spec_multi5: f32,
    #[serde(rename = "_SpecularHighlights")]
    pub specular_highlights: f32,
    #[serde(rename = "_SrcBlend")]
    pub src_blend: f32,
    #[serde(rename = "_TessEdgeLength")]
    pub tess_edge_length: f32,
    #[serde(rename = "_TessOutlineZOffset")]
    pub tess_outline_zoffset: f32,
    #[serde(rename = "_TessPidStart")]
    pub tess_pid_start: f32,
    #[serde(rename = "_TessStrength")]
    pub tess_strength: f32,
    #[serde(rename = "_TessType")]
    pub tess_type: f32,
    #[serde(rename = "_TessellationCheckInflectionOn")]
    pub tessellation_check_inflection_on: f32,
    #[serde(rename = "_TessellationOn")]
    pub tessellation_on: f32,
    #[serde(rename = "_TextureBiasWhenDithering")]
    pub texture_bias_when_dithering: f32,
    #[serde(rename = "_TextureLineSmoothness")]
    pub texture_line_smoothness: f32,
    #[serde(rename = "_TextureLineThickness")]
    pub texture_line_thickness: f32,
    #[serde(rename = "_UVSec")]
    pub uvsec: f32,
    #[serde(rename = "_UseBackFaceUV2")]
    pub use_back_face_uv2: f32,
    #[serde(rename = "_UseBumpMap")]
    pub use_bump_map: f32,
    #[serde(rename = "_UseClipPlane")]
    pub use_clip_plane: f32,
    #[serde(rename = "_UseClipping")]
    pub use_clipping: f32,
    #[serde(rename = "_UseCoolShadowColorOrTex")]
    pub use_cool_shadow_color_or_tex: f32,
    #[serde(rename = "_UseFaceMapNew")]
    pub use_face_map_new: f32,
    #[serde(rename = "_UseLightMapColorAO")]
    pub use_light_map_color_ao: f32,
    #[serde(rename = "_UseMaterial2")]
    pub use_material2: f32,
    #[serde(rename = "_UseMaterial3")]
    pub use_material3: f32,
    #[serde(rename = "_UseMaterial4")]
    pub use_material4: f32,
    #[serde(rename = "_UseMaterial5")]
    pub use_material5: f32,
    #[serde(rename = "_UseMaterialMasksTex")]
    pub use_material_masks_tex: f32,
    #[serde(rename = "_UseShadowRamp")]
    pub use_shadow_ramp: f32,
    #[serde(rename = "_UseShadowTransition")]
    pub use_shadow_transition: f32,
    #[serde(rename = "_UseToonLightMap")]
    pub use_toon_light_map: f32,
    #[serde(rename = "_UseToonSpecular")]
    pub use_toon_specular: f32,
    #[serde(rename = "_UseVertexColorAO")]
    pub use_vertex_color_ao: f32,
    #[serde(rename = "_UseVertexRampWidth")]
    pub use_vertex_ramp_width: f32,
    #[serde(rename = "_UsingDitherAlpha")]
    pub using_dither_alpha: f32,
    #[serde(rename = "_UtilityDisplay1")]
    pub utility_display1: f32,
    #[serde(rename = "_UtilityDisplay2")]
    pub utility_display2: f32,
    #[serde(rename = "_ZWrite")]
    pub zwrite: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MColors {
    #[serde(rename = "_ClipBoxPositionOffset")]
    pub clip_box_position_offset: Color,
    #[serde(rename = "_ClipBoxScale")]
    pub clip_box_scale: Color,
    #[serde(rename = "_ClipHighLightColor")]
    pub clip_high_light_color: Color,
    #[serde(rename = "_ClipPlane")]
    pub clip_plane: Color,
    #[serde(rename = "_Color")]
    pub color: Color,
    #[serde(rename = "_Color2")]
    pub color2: Color,
    #[serde(rename = "_Color3")]
    pub color3: Color,
    #[serde(rename = "_Color4")]
    pub color4: Color,
    #[serde(rename = "_Color5")]
    pub color5: Color,
    #[serde(rename = "_CoolShadowMultColor")]
    pub cool_shadow_mult_color: Color,
    #[serde(rename = "_CoolShadowMultColor2")]
    pub cool_shadow_mult_color2: Color,
    #[serde(rename = "_CoolShadowMultColor3")]
    pub cool_shadow_mult_color3: Color,
    #[serde(rename = "_CoolShadowMultColor4")]
    pub cool_shadow_mult_color4: Color,
    #[serde(rename = "_CoolShadowMultColor5")]
    pub cool_shadow_mult_color5: Color,
    #[serde(rename = "_ElementRimColor")]
    pub element_rim_color: Color,
    #[serde(rename = "_EmissionColor")]
    pub emission_color: Color,
    #[serde(rename = "_EmissionColor_MHY")]
    pub emission_color_mhy: Color,
    #[serde(rename = "_FaceBlushColor")]
    pub face_blush_color: Color,
    #[serde(rename = "_FirstShadowMultColor")]
    pub first_shadow_mult_color: Color,
    #[serde(rename = "_FirstShadowMultColor2")]
    pub first_shadow_mult_color2: Color,
    #[serde(rename = "_FirstShadowMultColor3")]
    pub first_shadow_mult_color3: Color,
    #[serde(rename = "_FirstShadowMultColor4")]
    pub first_shadow_mult_color4: Color,
    #[serde(rename = "_FirstShadowMultColor5")]
    pub first_shadow_mult_color5: Color,
    #[serde(rename = "_HitColor")]
    pub hit_color: Color,
    #[serde(rename = "_MTMapDarkColor")]
    pub mtmap_dark_color: Color,
    #[serde(rename = "_MTMapLightColor")]
    pub mtmap_light_color: Color,
    #[serde(rename = "_MTShadowMultiColor")]
    pub mtshadow_multi_color: Color,
    #[serde(rename = "_MTSharpLayerColor")]
    pub mtsharp_layer_color: Color,
    #[serde(rename = "_MTSpecularColor")]
    pub mtspecular_color: Color,
    #[serde(rename = "_MainTexTintColor")]
    pub main_tex_tint_color: Color,
    #[serde(rename = "_OutlineColor")]
    pub outline_color: Color,
    #[serde(rename = "_OutlineColor2")]
    pub outline_color2: Color,
    #[serde(rename = "_OutlineColor3")]
    pub outline_color3: Color,
    #[serde(rename = "_OutlineColor4")]
    pub outline_color4: Color,
    #[serde(rename = "_OutlineColor5")]
    pub outline_color5: Color,
    #[serde(rename = "_OutlineWidthAdjustScales")]
    pub outline_width_adjust_scales: Color,
    #[serde(rename = "_OutlineWidthAdjustZs")]
    pub outline_width_adjust_zs: Color,
    #[serde(rename = "_SingleColorOutputColor")]
    pub single_color_output_color: Color,
    #[serde(rename = "_SpecularColor")]
    pub specular_color: Color,
    #[serde(rename = "_TextureLineDistanceControl")]
    pub texture_line_distance_control: Color,
    #[serde(rename = "_TextureLineMultiplier")]
    pub texture_line_multiplier: Color,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MStringTagMap {
}
