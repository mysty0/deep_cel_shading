use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct MaterialPropertiesRoot {
    #[serde(rename = "m_Shader", default)]
    pub m_shader: MShader,
    #[serde(rename = "m_SavedProperties", default)]
    pub m_saved_properties: MSavedProperties,
    #[serde(rename = "m_StringTagMap", default)]
    pub m_string_tag_map: MStringTagMap,
    #[serde(rename = "m_Name", default)]
    pub m_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MShader {
    #[serde(rename = "m_FileID", default)]
    pub m_file_id: i64,
    #[serde(rename = "m_PathID", default)]
    pub m_path_id: i64,
    #[serde(rename = "IsNull", default)]
    pub is_null: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MSavedProperties {
    //#[serde(rename = "m_TexEnvs", default)]
    //pub m_tex_envs: MTexEnvs,
    #[serde(rename = "m_Ints", default)]
    pub m_ints: Value,
    #[serde(rename = "m_Floats", default)]
    pub m_floats: MFloats,
    #[serde(rename = "m_Colors", default)]
    pub m_colors: MColors,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MTexEnvs {
    #[serde(rename = "_AnimTextureQ", default)]
    pub anim_texture_q: Texture,
    #[serde(rename = "_AnimTextureT", default)]
    pub anim_texture_t: Texture,
    #[serde(rename = "_ClipAlphaTex", default)]
    pub clip_alpha_tex: Texture,
    #[serde(rename = "_FaceMapTex", default)]
    pub face_map_tex: Texture,
    #[serde(rename = "_LightMapTex", default)]
    pub light_map_tex: Texture,
    #[serde(rename = "_MTMap", default)]
    pub mtmap: Texture,
    #[serde(rename = "_MTSpecularRamp", default)]
    pub mtspecular_ramp: Texture,
    #[serde(rename = "_MainTex", default)]
    pub main_tex: Texture,
    #[serde(rename = "_MaterialMasksTex", default)]
    pub material_masks_tex: Texture,
    #[serde(rename = "_PackedShadowRampTex", default)]
    pub packed_shadow_ramp_tex: Texture,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    #[serde(rename = "m_Texture", default)]
    pub m_texture: TextureFile,
    #[serde(rename = "m_Scale", default)]
    pub m_scale: Vec2,
    #[serde(rename = "m_Offset", default)]
    pub m_offset: Vec2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureFile {
    #[serde(rename = "m_FileID", default)]
    pub m_file_id: i64,
    #[serde(rename = "m_PathID", default)]
    pub m_path_id: i64,
    #[serde(rename = "IsNull", default)]
    pub is_null: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vec2 {
    #[serde(rename = "X", default)]
    pub x: f32,
    #[serde(rename = "Y", default)]
    pub y: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MFloats {
    #[serde(rename = "_AnimBoneOffset", default)]
    pub anim_bone_offset: f32,
    #[serde(rename = "_AnimFPS", default)]
    pub anim_fps: f32,
    #[serde(rename = "_BumpScale", default)]
    pub bump_scale: f32,
    #[serde(rename = "_CharacterAmbientSensorColorOn", default)]
    pub character_ambient_sensor_color_on: f32,
    #[serde(rename = "_CharacterAmbientSensorForceShadowOn", default)]
    pub character_ambient_sensor_force_shadow_on: f32,
    #[serde(rename = "_CharacterAmbientSensorShadowOn", default)]
    pub character_ambient_sensor_shadow_on: f32,
    #[serde(rename = "_ClipAlphaHighLightScale", default)]
    pub clip_alpha_high_light_scale: f32,
    #[serde(rename = "_ClipAlphaThreshold", default)]
    pub clip_alpha_threshold: f32,
    #[serde(rename = "_ClipAlphaUVSet", default)]
    pub clip_alpha_uvset: f32,
    #[serde(rename = "_ClipBoxHighLightScale", default)]
    pub clip_box_high_light_scale: f32,
    #[serde(rename = "_ClipDissolveDirection", default)]
    pub clip_dissolve_direction: f32,
    #[serde(rename = "_ClipDissolveHightlightScale", default)]
    pub clip_dissolve_hightlight_scale: f32,
    #[serde(rename = "_ClipDissolveValue", default)]
    pub clip_dissolve_value: f32,
    #[serde(rename = "_ClipMethod", default)]
    pub clip_method: f32,
    #[serde(rename = "_ClipPlaneWorld", default)]
    pub clip_plane_world: f32,
    #[serde(rename = "_CullMode", default)]
    pub cull_mode: f32,
    #[serde(rename = "_DitherAlpha", default)]
    pub dither_alpha: f32,
    #[serde(rename = "_DrawBackFace", default)]
    pub draw_back_face: f32,
    #[serde(rename = "_ElementViewEleID", default)]
    pub element_view_ele_id: f32,
    #[serde(rename = "_EmissionScaler", default)]
    pub emission_scaler: f32,
    #[serde(rename = "_EmissionScaler1", default)]
    pub emission_scaler1: f32,
    #[serde(rename = "_EmissionScaler2", default)]
    pub emission_scaler2: f32,
    #[serde(rename = "_EmissionScaler3", default)]
    pub emission_scaler3: f32,
    #[serde(rename = "_EmissionScaler4", default)]
    pub emission_scaler4: f32,
    #[serde(rename = "_EmissionScaler5", default)]
    pub emission_scaler5: f32,
    #[serde(rename = "_EmissionStrengthLerp", default)]
    pub emission_strength_lerp: f32,
    #[serde(rename = "_FaceBlushStrength", default)]
    pub face_blush_strength: f32,
    #[serde(rename = "_FaceMapRotateOffset", default)]
    pub face_map_rotate_offset: f32,
    #[serde(rename = "_FaceMapSoftness", default)]
    pub face_map_softness: f32,
    #[serde(rename = "_HitColorFresnelPower", default)]
    pub hit_color_fresnel_power: f32,
    #[serde(rename = "_HitColorScaler", default)]
    pub hit_color_scaler: f32,
    #[serde(rename = "_InstanceData", default)]
    pub instance_data: f32,
    #[serde(rename = "_LightArea", default)]
    pub light_area: f32,
    #[serde(rename = "_MTMapBrightness", default)]
    pub mtmap_brightness: f32,
    #[serde(rename = "_MTMapTileScale", default)]
    pub mtmap_tile_scale: f32,
    #[serde(rename = "_MTSharpLayerOffset", default)]
    pub mtsharp_layer_offset: f32,
    #[serde(rename = "_MTShininess", default)]
    pub mtshininess: f32,
    #[serde(rename = "_MTSpecularAttenInShadow", default)]
    pub mtspecular_atten_in_shadow: f32,
    #[serde(rename = "_MTSpecularScale", default)]
    pub mtspecular_scale: f32,
    #[serde(rename = "_MTUseSpecularRamp", default)]
    pub mtuse_specular_ramp: f32,
    #[serde(rename = "_MainTexAlphaCutoff", default)]
    pub main_tex_alpha_cutoff: f32,
    #[serde(rename = "_MainTexAlphaUse", default)]
    pub main_tex_alpha_use: f32,
    #[serde(rename = "_MainTexColoring", default)]
    pub main_tex_coloring: f32,
    #[serde(rename = "_MaxOutlineZOffset", default)]
    pub max_outline_zoffset: f32,
    #[serde(rename = "_MetalMaterial", default)]
    pub metal_material: f32,
    #[serde(rename = "_OutlinePolygonOffsetFactor", default)]
    pub outline_polygon_offset_factor: f32,
    #[serde(rename = "_OutlinePolygonOffsetUnit", default)]
    pub outline_polygon_offset_unit: f32,
    #[serde(rename = "_OutlineType", default)]
    pub outline_type: f32,
    #[serde(rename = "_OutlineWidth", default)]
    pub outline_width: f32,
    #[serde(rename = "_PolygonOffsetFactor", default)]
    pub polygon_offset_factor: f32,
    #[serde(rename = "_PolygonOffsetUnit", default)]
    pub polygon_offset_unit: f32,
    #[serde(rename = "_Scale", default)]
    pub scale: f32,
    #[serde(rename = "_ShadowRampWidth", default)]
    pub shadow_ramp_width: f32,
    #[serde(rename = "_ShadowTransitionRange", default)]
    pub shadow_transition_range: f32,
    #[serde(rename = "_ShadowTransitionRange2", default)]
    pub shadow_transition_range2: f32,
    #[serde(rename = "_ShadowTransitionRange3", default)]
    pub shadow_transition_range3: f32,
    #[serde(rename = "_ShadowTransitionRange4", default)]
    pub shadow_transition_range4: f32,
    #[serde(rename = "_ShadowTransitionRange5", default)]
    pub shadow_transition_range5: f32,
    #[serde(rename = "_ShadowTransitionSoftness", default)]
    pub shadow_transition_softness: f32,
    #[serde(rename = "_ShadowTransitionSoftness2", default)]
    pub shadow_transition_softness2: f32,
    #[serde(rename = "_ShadowTransitionSoftness3", default)]
    pub shadow_transition_softness3: f32,
    #[serde(rename = "_ShadowTransitionSoftness4", default)]
    pub shadow_transition_softness4: f32,
    #[serde(rename = "_ShadowTransitionSoftness5", default)]
    pub shadow_transition_softness5: f32,
    #[serde(rename = "_Shininess", default)]
    pub shininess: f32,
    #[serde(rename = "_Shininess2", default)]
    pub shininess2: f32,
    #[serde(rename = "_Shininess3", default)]
    pub shininess3: f32,
    #[serde(rename = "_Shininess4", default)]
    pub shininess4: f32,
    #[serde(rename = "_Shininess5", default)]
    pub shininess5: f32,
    #[serde(rename = "_SpecMulti", default)]
    pub spec_multi: f32,
    #[serde(rename = "_SpecMulti2", default)]
    pub spec_multi2: f32,
    #[serde(rename = "_SpecMulti3", default)]
    pub spec_multi3: f32,
    #[serde(rename = "_SpecMulti4", default)]
    pub spec_multi4: f32,
    #[serde(rename = "_SpecMulti5", default)]
    pub spec_multi5: f32,
    #[serde(rename = "_TessEdgeLength", default)]
    pub tess_edge_length: f32,
    #[serde(rename = "_TessOutlineZOffset", default)]
    pub tess_outline_zoffset: f32,
    #[serde(rename = "_TessPidStart", default)]
    pub tess_pid_start: f32,
    #[serde(rename = "_TessStrength", default)]
    pub tess_strength: f32,
    #[serde(rename = "_TessType", default)]
    pub tess_type: f32,
    #[serde(rename = "_TessellationCheckInflectionOn", default)]
    pub tessellation_check_inflection_on: f32,
    #[serde(rename = "_TessellationOn", default)]
    pub tessellation_on: f32,
    #[serde(rename = "_TextureBiasWhenDithering", default)]
    pub texture_bias_when_dithering: f32,
    #[serde(rename = "_TextureLineSmoothness", default)]
    pub texture_line_smoothness: f32,
    #[serde(rename = "_TextureLineThickness", default)]
    pub texture_line_thickness: f32,
    #[serde(rename = "_UseBackFaceUV2", default)]
    pub use_back_face_uv2: f32,
    #[serde(rename = "_UseBumpMap", default)]
    pub use_bump_map: f32,
    #[serde(rename = "_UseClipPlane", default)]
    pub use_clip_plane: f32,
    #[serde(rename = "_UseClipping", default)]
    pub use_clipping: f32,
    #[serde(rename = "_UseCoolShadowColorOrTex", default)]
    pub use_cool_shadow_color_or_tex: f32,
    #[serde(rename = "_UseFaceMapNew", default)]
    pub use_face_map_new: f32,
    #[serde(rename = "_UseLightMapColorAO", default)]
    pub use_light_map_color_ao: f32,
    #[serde(rename = "_UseMaterial2", default)]
    pub use_material2: f32,
    #[serde(rename = "_UseMaterial3", default)]
    pub use_material3: f32,
    #[serde(rename = "_UseMaterial4", default)]
    pub use_material4: f32,
    #[serde(rename = "_UseMaterial5", default)]
    pub use_material5: f32,
    #[serde(rename = "_UseMaterialMasksTex", default)]
    pub use_material_masks_tex: f32,
    #[serde(rename = "_UseShadowRamp", default)]
    pub use_shadow_ramp: f32,
    #[serde(rename = "_UseShadowTransition", default)]
    pub use_shadow_transition: f32,
    #[serde(rename = "_UseToonLightMap", default)]
    pub use_toon_light_map: f32,
    #[serde(rename = "_UseToonSpecular", default)]
    pub use_toon_specular: f32,
    #[serde(rename = "_UseVertexColorAO", default)]
    pub use_vertex_color_ao: f32,
    #[serde(rename = "_UseVertexRampWidth", default)]
    pub use_vertex_ramp_width: f32,
    #[serde(rename = "_UsingDitherAlpha", default)]
    pub using_dither_alpha: f32,
    #[serde(rename = "_UtilityDisplay1", default)]
    pub utility_display1: f32,
    #[serde(rename = "_UtilityDisplay2", default)]
    pub utility_display2: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MColors {
    #[serde(rename = "_ClipBoxPositionOffset", default)]
    pub clip_box_position_offset: Color,
    #[serde(rename = "_ClipBoxScale", default)]
    pub clip_box_scale: Color,
    #[serde(rename = "_ClipHighLightColor", default)]
    pub clip_high_light_color: Color,
    #[serde(rename = "_ClipPlane", default)]
    pub clip_plane: Color,
    #[serde(rename = "_Color", default)]
    pub color: Color,
    #[serde(rename = "_Color2", default)]
    pub color2: Color,
    #[serde(rename = "_Color3", default)]
    pub color3: Color,
    #[serde(rename = "_Color4", default)]
    pub color4: Color,
    #[serde(rename = "_Color5", default)]
    pub color5: Color,
    #[serde(rename = "_CoolShadowMultColor", default)]
    pub cool_shadow_mult_color: Color,
    #[serde(rename = "_CoolShadowMultColor2", default)]
    pub cool_shadow_mult_color2: Color,
    #[serde(rename = "_CoolShadowMultColor3", default)]
    pub cool_shadow_mult_color3: Color,
    #[serde(rename = "_CoolShadowMultColor4", default)]
    pub cool_shadow_mult_color4: Color,
    #[serde(rename = "_CoolShadowMultColor5", default)]
    pub cool_shadow_mult_color5: Color,
    #[serde(rename = "_ElementRimColor", default)]
    pub element_rim_color: Color,
    #[serde(rename = "_EmissionColor_MHY", default)]
    pub emission_color_mhy: Color,
    #[serde(rename = "_FaceBlushColor", default)]
    pub face_blush_color: Color,
    #[serde(rename = "_FirstShadowMultColor", default)]
    pub first_shadow_mult_color: Color,
    #[serde(rename = "_FirstShadowMultColor2", default)]
    pub first_shadow_mult_color2: Color,
    #[serde(rename = "_FirstShadowMultColor3", default)]
    pub first_shadow_mult_color3: Color,
    #[serde(rename = "_FirstShadowMultColor4", default)]
    pub first_shadow_mult_color4: Color,
    #[serde(rename = "_FirstShadowMultColor5", default)]
    pub first_shadow_mult_color5: Color,
    #[serde(rename = "_HitColor", default)]
    pub hit_color: Color,
    #[serde(rename = "_MTMapDarkColor", default)]
    pub mtmap_dark_color: Color,
    #[serde(rename = "_MTMapLightColor", default)]
    pub mtmap_light_color: Color,
    #[serde(rename = "_MTShadowMultiColor", default)]
    pub mtshadow_multi_color: Color,
    #[serde(rename = "_MTSharpLayerColor", default)]
    pub mtsharp_layer_color: Color,
    #[serde(rename = "_MTSpecularColor", default)]
    pub mtspecular_color: Color,
    #[serde(rename = "_MainTexTintColor", default)]
    pub main_tex_tint_color: Color,
    #[serde(rename = "_OutlineColor", default)]
    pub outline_color: Color,
    #[serde(rename = "_OutlineColor2", default)]
    pub outline_color2: Color,
    #[serde(rename = "_OutlineColor3", default)]
    pub outline_color3: Color,
    #[serde(rename = "_OutlineColor4", default)]
    pub outline_color4: Color,
    #[serde(rename = "_OutlineColor5", default)]
    pub outline_color5: Color,
    #[serde(rename = "_OutlineWidthAdjustScales", default)]
    pub outline_width_adjust_scales: Color,
    #[serde(rename = "_OutlineWidthAdjustZs", default)]
    pub outline_width_adjust_zs: Color,
    #[serde(rename = "_SingleColorOutputColor", default)]
    pub single_color_output_color: Color,
    #[serde(rename = "_SpecularColor", default)]
    pub specular_color: Color,
    #[serde(rename = "_TextureLineDistanceControl", default)]
    pub texture_line_distance_control: Color,
    #[serde(rename = "_TextureLineMultiplier", default)]
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
pub struct MStringTagMap {}
