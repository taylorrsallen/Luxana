#import bevy_pbr::{
    mesh_functions::mesh_position_local_to_clip,
    pbr_functions::alpha_discard,
    pbr_fragment::pbr_input_from_standard_material,
}

#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
}

struct InstVertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_pos_scale: vec4<f32>,
};

@vertex
fn vertex(vertex: InstVertex) -> VertexOutput {
    let position = vertex.position * 0.5 + vertex.i_pos_scale.xyz;
    var out: VertexOutput;
    // NOTE: Passing 0 as the instance_index to get_model_matrix() is a hack
    // for this example as the instance_index builtin would map to the wrong
    // index in the Mesh array. This index could be passed in via another
    // uniform instead but it's unnecessary for the example.
    out.position = mesh_position_local_to_clip(
        mat4x4<f32>(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ),
        vec4<f32>(position, 1.0)
    );

    // out.color = vec4<f32>(0.8, 0.8, 0.8, 1.0);
    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {

    var out: FragmentOutput;
    out.color = vec4<f32>(0.5, 0.5, 0.5, 1.0);

    return out;
}