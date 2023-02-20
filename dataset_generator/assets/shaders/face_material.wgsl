struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var light_map: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let light = vec4<f32>(4.0, 8.0, 4.0);

    return material.color * textureSample(base_color_texture, base_color_sampler, uv) * dot(normalize(light), world_normal);
}