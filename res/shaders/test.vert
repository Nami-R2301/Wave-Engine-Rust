#version 420

// Outputs.
struct Vertex_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
};

layout (std140, binding = 0) uniform u_view_projection
{
    mat4 m_matrix;
} U_View_projection_matrix;

layout (std140, binding = 1) uniform u_model_matrix
{
    mat4 m_matrix;
} U_Model_matrix;

layout (location = 0) in uint in_entity_ID;
layout (location = 1) in vec3 in_position;
layout (location = 2) in vec3 in_normal;
layout (location = 3) in vec4 in_color;
layout (location = 4) in vec2 in_tex_coords;

layout (location = 0) flat out uint vout_entity_ID;
layout (location = 1) out Vertex_data_s vout_vertex_data;

void main() {
    gl_Position = U_View_projection_matrix.m_matrix * (U_Model_matrix.m_matrix * vec4(in_position, 1.0));
    vout_entity_ID = in_entity_ID;
    vout_vertex_data.vout_normal = in_normal;
    vout_vertex_data.vout_tex_coords = in_tex_coords;
    vout_vertex_data.vout_frag_color = in_color;
}