#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::utils
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

@group(1) @binding(0)
var matcap_tex: texture_2d<f32>;
@group(1) @binding(1)
var matcap_sampler: sampler;

fn calculate_view(
    world_position: vec4<f32>,
) -> vec3<f32> {
    return normalize(view.world_position.xyz - world_position.xyz);
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
    @builtin(position) position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {

    let view_direction = calculate_view(world_position);

    let view_normal = view.view * vec4<f32>(world_normal, 0.0);

    //let matcapuv_detail = view_normal.xyz * vec3<f32>(-1.0, -1.0, 1.0);
    //let matcapuv_base = (view.view * vec4<f32>(view_direction, 0.0)).xyz * vec3<f32>(-1.0, -1.0, 1.0) ;
    //let matcapuvs = matcapuv_base * dot(matcapuv_base, matcapuv_detail) / matcapuv_base.z - matcapuv_detail;

    // let matcapuvs = vec2<f32>(dot(vec4(world_normal, 1.0), view.view[0]), dot(vec4(world_normal, 1.0), view.view[1]));
    // //normalize [-1; 1] -> [0; 1]
    // let matcapuvs = (matcapuvs + 1.0)/2.0;
    // let metal = textureSample(matcap_tex, matcap_sampler, matcapuvs.xy);



    let light_pos = vec3<f32>(100.0, 100.0, 100.0);
    let N = normalize(world_normal);
    let L = normalize(light_pos - world_position.xyz);
    // Lambert's cosine law
    let lambertian = max(dot(N, L), 0.0);

    let R = reflect(-L, N);      // Reflected light vector
    let V = normalize(view.world_position - world_position.xyz); // Vector to viewer
    // Compute the specular term
    let specAngle = max(dot(R, V), 0.0);
    let specular = pow(specAngle, 80.0);

    let color = vec4<f32>(1.0 * vec3<f32>(0.921, 0.368, 0.156) * 0.9 + 
    1.0 * lambertian * vec3<f32>(0.921, 0.368, 0.156) + 
    1.0 * specular * vec3<f32>(1.0, 0.7, 0.3), 
    1.0);

    return vec4(color.xyz, 1.0);
}