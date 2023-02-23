struct Direction {
    forward: vec3<f32>,
    right: vec3<f32>,
}

@group(1) @binding(1)
var diffuse: texture_2d<f32>;
@group(1) @binding(2)
var diffuse_sampler: sampler;

@group(1) @binding(3)
var light_map: texture_2d<f32>;
@group(1) @binding(4)
var light_map_sampler: sampler;

@group(1) @binding(5)
var shadow_map: texture_2d<f32>;
@group(1) @binding(6)
var shadow_map_sampler: sampler;

@group(1) @binding(7)
var shadow_ramp: texture_2d<f32>;
@group(1) @binding(8)
var shadow_ramp_sampler: sampler;

@group(1) @binding(9)
var<uniform> head_direction: Direction;

@group(1) @binding(9)
var<uniform> day_night_cycle: f32;


@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let light = vec3<f32>(4.0, 8.0, 4.0);
    let color = textureSample(diffuse, diffuse_sampler, uv);
    let light_mapped = textureSample(light_map, light_map_sampler, uv);
    let light = dot(normalize(light), world_normal) * 0.5 + 0.5;
    return vec4<f32>(color.x, color.y, color.z, 1.0f) * light;
}