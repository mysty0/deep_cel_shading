@group(1) @binding(0)
var light_map: texture_2d<f32>;
@group(1) @binding(1)
var light_map_sampler: sampler;

@group(1) @binding(2)
var diffuse: texture_2d<f32>;
@group(1) @binding(3)
var diffuse_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let light = vec3<f32>(4.0, 8.0, 4.0);
    let color = textureSample(diffuse, diffuse_sampler, uv);
    let light_mapped = textureSample(light_map, light_map_sampler, uv);
    return vec4<f32>(color.x, color.y, color.z, 1.0f) * light_mapped.a;// * dot(normalize(light), world_normal);
}