#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import cel::utils

#import bevy_pbr::mesh_functions


struct Direction {
    forward: vec3<f32>,
    right: vec3<f32>,
}

struct ShadowRamp {
    width: f32,
    day_mult_colors: array<vec4<f32>, 5>,
    night_mult_colors: array<vec4<f32>, 5>,
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

struct ShadowRampValue {
    day_mult_color: vec4<f32>,
    night_mult_color: vec4<f32>,
    transition_range: f32,
    transition_softness: f32,
}

fn get_shadow_ramp_value(ramp: ShadowRamp, index: i32) -> ShadowRampValue {
    var value = ShadowRampValue(
        ramp.day_mult_colors[0], 
        ramp.night_mult_colors[0],
        ramp.transition_range1,
        ramp.transition_softness1,
    );

    if index == 2 {
        value.day_mult_color = ramp.day_mult_colors[0];
        value.night_mult_color = ramp.night_mult_colors[0];
        value.transition_range = ramp.transition_range2;
        value.transition_softness = ramp.transition_softness2;
    }
    if index == 3 {
        value.day_mult_color = ramp.day_mult_colors[1];
        value.night_mult_color = ramp.night_mult_colors[2];
        value.transition_range = ramp.transition_range3;
        value.transition_softness = ramp.transition_softness3;
    }
    if index == 4 {
        value.day_mult_color = ramp.day_mult_colors[3];
        value.night_mult_color = ramp.night_mult_colors[3];
        value.transition_range = ramp.transition_range4;
        value.transition_softness = ramp.transition_softness4;
    }
    if index == 5 {
        value.day_mult_color = ramp.day_mult_colors[4];
        value.night_mult_color = ramp.night_mult_colors[4];
        value.transition_range = ramp.transition_range5;
        value.transition_softness = ramp.transition_softness5;
    }

    return value;
}

struct GlobalSpecular {
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

struct GlobalSpecularValue {
    shininess: f32,
    specular_multi: f32,
}

struct MaterialProperties {
    head_direction: Direction,
    shadow_ramp_values: ShadowRamp,
    global_specular: GlobalSpecular,

    use_materials: vec4<f32>, //1 1 1 1 1
    metal_map_light_color: vec4<f32>,
    metal_map_dark_color: vec4<f32>,
    metal_map_shadow_multi_color: vec4<f32>,
    metal_map_sharp_layer_color: vec4<f32>,
    metal_map_specular_color: vec4<f32>,
    specular_color: vec4<f32>,
    hit_color: vec4<f32>,

    day_night_cycle: f32,

    use_shadow_ramp_texture: f32,

    light_area: f32, //0.55
    flip_light_map: f32, //0
    face_map_softness: f32, //0.1

    use_ligth_map_color_ao: f32, //1
    use_vertex_color_ao: f32, //1

    normal_map_scale: f32, //0.2

    use_normal_map: f32, //1

    use_back_space_uv: f32, //1

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

@group(1) @binding(0)
var diffuse_tex: texture_2d<f32>;
@group(1) @binding(1)
var diffuse_sampler: sampler;

@group(1) @binding(2)
var face_light_map_tex: texture_2d<f32>;
@group(1) @binding(3)
var face_light_map_sampler: sampler;

@group(1) @binding(4)
var light_map_tex: texture_2d<f32>;
@group(1) @binding(5)
var light_map_sampler: sampler;

@group(1) @binding(6)
var shadow_ramp_tex: texture_2d<f32>;
@group(1) @binding(7)
var shadow_ramp_sampler: sampler;

@group(1) @binding(8)
var normal_map_tex: texture_2d<f32>;
@group(1) @binding(9)
var normal_map_sampler: sampler;

@group(1) @binding(10)
var metal_map_tex: texture_2d<f32>;
@group(1) @binding(11)
var metal_map_sampler: sampler;

@group(1) @binding(12)
var<uniform> properties: MaterialProperties;

fn get_specular_value(material_id: i32) -> GlobalSpecularValue {
    let gs = properties.global_specular;
    var value = GlobalSpecularValue(gs.shininess1, gs.specular_multi1);
    if material_id == 2 {
        value.shininess = gs.shininess2;
        value.specular_multi = gs.specular_multi2;
    }
    if material_id == 3 {
        value.shininess = gs.shininess3;
        value.specular_multi = gs.specular_multi3;
    }
    if material_id == 4 {
        value.shininess = gs.shininess4;
        value.specular_multi = gs.specular_multi4;
    }
    if material_id == 5 {
        value.shininess = gs.shininess5;
        value.specular_multi = gs.specular_multi5;
    }

    return value;
}

// @group(1) @binding(25)
// var<uniform> use_metal_material: f32; //1
// @group(1) @binding(25)
// var<uniform> metal_map_tile_scale: f32; //1
// @group(1) @binding(25)
// var<uniform> use_metal_material: f32; //1
// @group(1) @binding(25)
// var<uniform> use_metal_material: f32; //1
// @group(1) @binding(25)
// var<uniform> use_metal_material: f32; //1

// _MTMapTileScale
// _MTMapBrightness
// _MTMapLightColor
// _MTShadowMultiColor


fn saturate(value: f32) -> f32 {
    return clamp(value,0.0,1.0);
}

fn saturate4(value: vec4<f32>) -> vec4<f32> {
    return clamp(value, vec4<f32>(0.0), vec4<f32>(1.0));
}

fn calculate_view(
    world_position: vec4<f32>,
) -> vec3<f32> {
    return normalize(view.world_position.xyz - world_position.xyz);
}

fn texel_size(texture: texture_2d<f32>) -> vec4<f32> {
    let size = vec2<f32>(textureDimensions(texture));
    return vec4<f32>(1.0/size.x, 1.0/size.y, size);
}

fn shadow_ramp_face(material_id: i32, factor: f32) -> vec4<f32> {
    if properties.use_shadow_ramp_texture > 0.0 {
        let day = textureSample(shadow_ramp_tex, shadow_ramp_sampler, vec2<f32>(factor, (((6.0 - f32(material_id)) - 1.0) * 0.1) + 0.05));
        let night = textureSample(shadow_ramp_tex, shadow_ramp_sampler, vec2<f32>(factor, (((6.0 - f32(material_id)) - 1.0) * 0.1) + 0.05 + 0.5));

        return mix(day, night, properties.day_night_cycle);
    } else {
        return mix(properties.shadow_ramp_values.day_mult_colors[0], properties.shadow_ramp_values.day_mult_colors[0], properties.day_night_cycle);
    }
}

fn shadow_ramp(material_id: i32, factor: f32, occlusion: f32, shadow_ramp_multiplier: f32) -> vec4<f32> {

    //half occlusion = ((_UseLightMapColorAO != 0) ? lightmapTex.g : 0.5) * ((_UseVertexColorAO != 0) ? i.vertexcol.r : 1.0);

    // vector<fixed, 4> ShadowDay = _FirstShadowMultColor;
    //         vector<fixed, 4> ShadowNight = _CoolShadowMultColor;

    //         ShadowFinal = lerp(ShadowDay, ShadowNight, _DayOrNight);

    if properties.use_shadow_ramp_texture > 0.0 {
        let width = shadow_ramp_multiplier * 2.0 * properties.shadow_ramp_values.width;

        let occlusion = smoothstep(0.01, 0.4, occlusion);
        let factor = mix(0.0, factor, saturate(factor));
        let lit_factor = factor < properties.light_area;

        let factor = 1.0 - ((properties.light_area - factor) / properties.light_area) / width;
        let factor = saturate(factor);

        let day = textureSample(shadow_ramp_tex, shadow_ramp_sampler, vec2<f32>(factor, (((6.0 - f32(material_id)) - 1.0) * 0.1) + 0.05));
        let night = textureSample(shadow_ramp_tex, shadow_ramp_sampler, vec2<f32>(factor, (((6.0 - f32(material_id)) - 1.0) * 0.1) + 0.05 + 0.5));

        let shadow = mix(day, night, properties.day_night_cycle);

        // switch between 1 and ramp edge like how the game does it, also make eyes always lit
        //ShadowFinal = (litFactor && lightmapTex.g < 0.95) ? ShadowFinal : 1;

        return select(vec4<f32>(1.0), mix(shadow, vec4<f32>(1.0), factor), lit_factor);
    } else {
        let factor = (factor + occlusion) * 0.5;
        let factor = select(factor, 1.0, occlusion > 0.95);
        let factor = select(factor, 0.0, occlusion < 0.05);

        let value = get_shadow_ramp_value(properties.shadow_ramp_values, material_id);
        let tansition_range = value.transition_range;//shadow_ramp_values.transition_range[material_id];
        let tansition_softness = value.transition_softness;//shadow_ramp_values.transition_range[material_id];
        let cool_color = value.night_mult_color;//shadow_ramp_values.night_mult_colors[material_id];
        let first_color = value.day_mult_color;//.day_mult_colors[material_id];
        // let tansition_range = shadow_ramp_values.transition_range[material_id];
        // let tansition_softness = shadow_ramp_values.transition_range[material_id];
        // let cool_color = shadow_ramp_values.night_mult_colors[material_id];
        // let first_color = shadow_ramp_values.day_mult_colors[material_id];

        var factor = factor;
        if factor < properties.light_area {
            factor = (-factor + properties.light_area) / tansition_range;
            let lit = factor >= 1.0;
            factor = exp2(log2(factor + 0.01) * tansition_softness);
            factor = min(factor, 1.0);
            factor = select(factor, 1.0, lit);
        } else {
            factor = 0.0;
        }

        let shadow_day = factor * first_color;
        let shadow_night = factor * cool_color;
        let shadow = mix(shadow_day, shadow_day, properties.day_night_cycle);

        return mix(vec4(1.0), shadow, factor);
    }
}

fn face_color(uv: vec2<f32>, vertex_color: vec4<f32>, world_normal: vec3<f32>, light_dir: vec3<f32>) -> vec4<f32> {
    let color = textureBicubic(diffuse_tex, diffuse_sampler, uv);

    let light_map = textureBicubic(face_light_map_tex, face_light_map_sampler, uv);
    let light_map_mirrored = textureBicubic(face_light_map_tex, face_light_map_sampler, vec2(1.0 - uv.x, uv.y));
    let face_map = textureBicubic(light_map_tex, light_map_sampler, uv);

    let light_2d = normalize(light_dir.xz);
    let forward_light = dot(light_2d, properties.head_direction.forward.xz);
    // remap both dot products from { -1, 1 } to { 0, 1 } and invert
    let forward_light = 1.0 - (forward_light * 0.5 + 0.5);
    //let forward_light = (forward_light * 0.5 + 0.5);

    let right_light = dot(light_2d, properties.head_direction.right.xz);
    let right_light = select(1.0 - (right_light * 0.5 + 0.5), right_light * 0.5 + 0.5, properties.flip_light_map > 0.0);

    let light_map_dir = select(light_map, light_map_mirrored, right_light <= 0.5);
    
    // use FdotL to drive the face SDF, make sure forward_light has a maximum of 0.999 so that it doesn't glitch
    let shadow_range = min(0.999, forward_light);
    let shadow_range = pow(shadow_range, pow((2.0 - (properties.light_area + 0.50)), 3.0));

    let face_light = smoothstep(shadow_range - properties.face_map_softness, shadow_range + properties.face_map_softness, light_map_dir.r);
    let face_light = face_light + face_map.w * (1.0 - forward_light);
    let lit_factor = 1.0 - face_light;

    let light = dot(light_dir, world_normal);
    //let smooth_light = smoothstep(0.0, light_smooth, light);

    let occlusion = select(0.5, face_map.g, properties.use_ligth_map_color_ao > 0.0) * select(1.0, vertex_color.r, properties.use_vertex_color_ao > 0.0);
    //let shadow = shadow_ramp_face(1, face_light);
    let shadow = shadow_ramp(2, face_light, occlusion/2.0, vertex_color.g);
    //let shadow = mix(shadow, vec4<f32>(1.0), face_light);

    return color *shadow;//color * shadow;//vec4<f32>(face_light);//
}

fn standart_cel_color(
    world_position: vec4<f32>, 
    uv: vec2<f32>, 
    normal_map_uv: vec2<f32>, 
    vertex_color: vec4<f32>,
    world_normal: vec3<f32>,
    light_dir: vec3<f32>,
    light_position: vec3<f32>,
    is_front: bool,
    frag_coord: vec4<f32>,
) -> vec4<f32> {
    let uv = select(normal_map_uv, uv, is_front || !(properties.use_back_space_uv > 0.0));

    let color = textureBicubic(diffuse_tex, diffuse_sampler, uv);

    let light_map = textureBicubic(light_map_tex, light_map_sampler, uv);

    let id_mask = light_map.w;

    let material_id = i32(id_mask * 5.0);
    let material_id = select(1, material_id, properties.use_materials[material_id] > 0.0);

    var normal = world_normal;
    if properties.use_normal_map > 0.0 {
        // let mapped_normal = textureBicubic(normal_map_tex, normal_map_sampler, uv);

        // let denorm_tangent = dpdx(uv.y) * dpdy(world_normal) - dpdx(world_normal) * dpdy(uv.y);
        // let tangent = normalize(denorm_tangent - world_normal * dot(world_normal, denorm_tangent));
        // let bitangent = cross(world_normal, tangent);

        // normal = normalize(mat3x3(tangent, bitangent, world_normal) * mapped_normal.xyz);

        let bump = textureBicubic(normal_map_tex, normal_map_sampler, uv);
        var new_normal = vec3<f32>();
        var modified_normal = vec4<f32>();
        modified_normal = bump;

        new_normal = vec3<f32>(modified_normal.xy * 2.0 - 1.0, 0.0);
        new_normal.z = max(1.0 - min(properties.normal_map_scale, 0.95), 0.001);
        // Z??
        let tmp = new_normal * inverseSqrt(dot(new_normal, new_normal));
        modified_normal.x = tmp.x;
        modified_normal.y = tmp.y;
        modified_normal.w = tmp.z;

        var ddx = dpdx(world_position.xyz);
        var ddy = dpdy(world_position.xyz);

        var dhdx = vec3<f32>();
        var dhdy = vec3<f32>();
        dhdx = vec3<f32>(dpdx(uv), 0.0);
        dhdy = vec3<f32>(dpdy(uv), 0.0);

        dhdy.z = dhdx.y;
        dhdx.z = dhdy.x;

        new_normal = vec3<f32>(dot(dhdx.xz, dhdy.yz));
        var recalcTangent = -vec3<f32>(vec3<f32>(0.0) < new_normal) + vec3<f32>(new_normal < vec3<f32>(0.0));
        let tmp = vec2<f32>(recalcTangent.xy) * dhdy.yz;
        dhdx.x = tmp.x;
        dhdx.y = tmp.y;
        dhdy *= -dhdx.y;
        dhdx = dhdx * dhdx.x + dhdy;
        let normalCreationBuffer = inverseSqrt(dot(dhdx, dhdx));
        dhdx *= normalCreationBuffer;
        var normalCreationBuffer = world_normal;
        dhdy = normalCreationBuffer.zxy * dhdx.yzx;
        dhdy = normalCreationBuffer.yzx * dhdx.zxy - dhdy.xyz;
        dhdy *= -recalcTangent;
        dhdy *= modified_normal.y;
        dhdx = modified_normal.x * dhdx + dhdy;
        let tmp = modified_normal.www * normalCreationBuffer + dhdx;
        modified_normal.x = tmp.x;
        modified_normal.y = tmp.y;
        modified_normal.w = tmp.z;
        recalcTangent = vec3<f32>(inverseSqrt(dot(modified_normal.xyw, modified_normal.xyw)));
        let tmp = modified_normal.xyw * recalcTangent;
        modified_normal.x = tmp.x;
        modified_normal.y = tmp.y;
        modified_normal.w = tmp.z;

        if 0.99 >= modified_normal.w {
            normalCreationBuffer = modified_normal.xyw;
        }

        normal = normalCreationBuffer;
    }

    let light = dot(normal, light_dir);
    // remap from { -1, 1 } to { 0, 1 }
    let light = light * 0.5 + 0.5;

    //specular
    let view_postion = calculate_view(world_position);
    let light_pos = normalize(light_position + view_postion);
    let metal_specular = dot(normal, light_pos);
    
    let occlusion = select(0.5, light_map.g, properties.use_ligth_map_color_ao > 0.0) * select(1.0, vertex_color.r, properties.use_vertex_color_ao > 0.0);

    let shadow = shadow_ramp(
        material_id, 
        light, 
        occlusion, 
        vertex_color.g
    );

    let metal_factor = f32(light_map.r > 0.9) * properties.use_metal_map;
    let view_normal = view.view * vec4<f32>(normal, 1.0);
    // https://github.com/poiyomi/PoiyomiToonShader/blob/master/_PoiyomiShaders/Shaders/8.0/Poiyomi.shader#L8397
    let matcapuv_detail = view_normal.xyz * vec3<f32>(-1.0, -1.0, 1.0);
    let matcapuv_base = (view.view * vec4<f32>(view_postion, 1.0)).xyz * vec3<f32>(-1.0, -1.0, 1.0) + vec3<f32>(0.0, 0.0, 1.0);
    let matcapuvs = matcapuv_base * dot(matcapuv_base, matcapuv_detail) / matcapuv_base.z - matcapuv_detail;

    // offset UVs to middle and apply _MTMapTileScale
    let matcapuvs = vec2<f32>(matcapuvs.x * properties.metal_map_tile_scale, matcapuvs.y) * 0.5 + vec2<f32>(0.5, 0.5);

    // sample matcap texture with newly created UVs
    let metal = textureSample(metal_map_tex, metal_map_sampler, matcapuvs);
    
    // prevent metallic matcap from glowing
    let metal = saturate4(metal * properties.metal_map_brightness);
    let metal = mix(properties.metal_map_dark_color, properties.metal_map_light_color, metal);
    
    // apply _MTShadowMultiColor ONLY to shaded areas
    let metal = mix(metal * properties.metal_map_shadow_multi_color, metal, saturate(light));

    let specular_light_dir = normalize(view_postion + light_position);
    let specular_light = dot(normal, specular_light_dir);
    let metal_specular = saturate(pow(specular_light, properties.metal_map_shininess) * properties.metal_map_specular_scale);
    var metal_specular_color = vec4<f32>(metal_specular);
    if properties.metal_map_sharp_layer_offset < metal_specular {
        metal_specular_color = properties.metal_map_sharp_layer_color;
    } else {
        metal_specular_color *= properties.metal_map_specular_color;
        metal_specular_color *= light_map.z;
    }

    // apply _MTSpecularAttenInShadow ONLY to shaded areas
    metal_specular_color = mix(metal_specular_color * properties.metal_map_specular_atten_in_shadow, vec4<f32>(metal_specular), saturate(light));

    let specular_value = get_specular_value(material_id);
    let specular = pow(specular_light, specular_value.shininess);
    let specular = f32((1.0 - light_map.b) < specular);
    let specular = saturate4(properties.specular_color * vec4<f32>(specular * specular_value.specular_multi * light_map.r));
    
    var color = color.xyz;
    if metal_factor > 0.0 {
        color = color * metal.xyz + metal_specular_color.xyz; 
    } else {
        color = color * shadow.xyz + specular.xyz;
    }

    return vec4<f32>(color, 1.0);// * light;
}


struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(2) tangent: vec4<f32>,
#endif
    @location(3) uv: vec2<f32>,
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef VERTEX_NORMAL_MAP_UV
    @location(5) normal_map_uv: vec2<f32>,
#endif
};

#ifdef EMPTY
struct Vertex1{
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(2) tangent: vec4<f32>,
#endif
    @location(3) uv: vec2<f32>,
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef VERTEX_NORMAL_MAP_UV
    //@location(5) normal_map_uv: vec2<f32>,
#endif
};
@vertex
fn vertex(v: Vertex) -> @location(0) f32 { 
    return 0.0;
}

@fragment
fn fragment(
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(2) world_tangent: vec4<f32>,
#endif
    @location(3) uv: vec2<f32>,
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef VERTEX_NORMAL_MAP_UV
    //@location(5) normal_map_uv: vec2<f32>,
#endif
) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0);
}

#else

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(2) world_tangent: vec4<f32>,
#endif
    @location(3) uv: vec2<f32>,
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef VERTEX_NORMAL_MAP_UV
    @location(5) normal_map_uv: vec2<f32>,
#endif
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var model = mesh.model;
    var out: VertexOutput;
    var model = mesh.model;

    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
    out.uv = vertex.uv;
    out.clip_position = mesh_position_world_to_clip(out.world_position);


    #ifdef VERTEX_TANGENTS
        out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
    #endif

    #ifdef VERTEX_COLORS
        out.color = vertex.color;
    #endif

    #ifdef VERTEX_NORMAL_MAP_UV
        out.normal_map_uv = vertex.normal_map_uv;
    #endif

    return out;
}

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(2) world_tangent: vec4<f32>,
#endif
    @location(3) uv: vec2<f32>,
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifndef FACE
    @location(5) normal_map_uv: vec2<f32>,
#endif
) -> @location(0) vec4<f32> {

  //  let light_smooth = 0.1;
    
    //let light = vec3<f32>(-4.0, -8.0, 4.0);
    //let light_dir = light - world_position.xyz;
    //let light_dir = normalize(light_dir);

    let light = point_lights.data[0].position_radius.xyz;
    let light_dir = light - world_position.xyz;
    let light_dir = normalize(light_dir);

#ifdef FACE
    let color = face_color(uv, color, world_normal, light_dir);
#else

    // let world_tangent = vec4<f32>();
#ifndef VERTEX_COLORS
    let color = vec4<f32>();
#endif

    // let is_front = true;
    // let frag_coord = vec4<f32>();
    // let normal_map_uv = vec2<f32>();

    let color = standart_cel_color(
        world_position, 
        uv, 
        normal_map_uv, 
        color,
        world_normal,
        light_dir,
        light,
        is_front,
        frag_coord
    );
#endif

    //todo: use modified normal
    let frensel = vec3<f32>(inverseSqrt(dot(world_normal, world_normal))) * world_normal;

    //todo: double check if its right view direction
    let view_postion = calculate_view(world_position);
    let frensel_factor = 1.0 - saturate(dot(frensel, view_postion));
    let frensel_factor = max(frensel_factor, 9.99999975e-05);
    let frensel_factor = pow(frensel_factor, properties.hit_color_fresnel_power);

    let fresnel = properties.hit_color.xyz * vec3<f32>(frensel_factor * properties.hit_color_scaler);

    //  half rimLight = calculateRimLight(i.normal, i.screenPos, _RimLightIntensity, 
    //                                   _RimLightThickness, 1.0 - litFactor);

    // // rim light mustn't appear in backfaces
    // rimLight *= frontFacing;

    var color = color.xyz;

    color += f32(properties.use_fresnel != 0.0) * fresnel;

    // apply rim light
    //color.xyz = (_RimLightType != 0) ? ColorDodge(rimLight, finalColor.xyz) : finalColor.xyz + rimLight;

    return vec4<f32>(color, 1.0);
    // #ifndef FACE
    // return vec4<f32>(normal_map_uv.x, normal_map_uv.y, 1.0, 1.0);
    // #else

    // return vec4<f32>(uv.x, uv.y, 1.0, 1.0);//vec4<f32>(color.x, color.y, color.z, 1.0f);
    // #endif
}

#endif