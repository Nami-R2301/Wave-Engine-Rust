#version 330 core

// Outputs.
struct Vertex_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
};

// < 420 so we cannot bind blocks in shaders.
layout (std140) uniform ubo_camera
{
    mat4 m_view;
    mat4 m_projection;
} Ubo_camera;
layout (std140) uniform ubo_model
{
    mat4 m_matrix;
} Ubo_model;

layout (location = 0) in uint in_entity_ID;
layout (location = 1) in vec3 in_position;
layout (location = 2) in vec3 in_normal;
layout (location = 3) in vec4 in_color;
layout (location = 4) in vec2 in_tex_coords;


flat out uint vout_entity_ID;
out Vertex_data_s vout_vertex_data;

void main() {
    gl_Position = Ubo_camera.m_projection * Ubo_camera.m_view * (Ubo_model.m_matrix * vec4(in_position, 1.0));
    vout_entity_ID = in_entity_ID;
    vout_vertex_data.vout_normal = in_normal;
    vout_vertex_data.vout_tex_coords = in_tex_coords;
    vout_vertex_data.vout_frag_color = in_color;
}