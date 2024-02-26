#version 330 core

// Outputs.
struct Vertex_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
};

#ifdef Vulkan
// We are compiling for SPIR-V
layout (std140) uniform ubo_camera
{
    mat4 m_view;
    mat4 m_projection;
} Ubo_camera;
#else
// Classic GLSL
uniform mat4 u_view_projection = mat4(0.0f);
#endif

#ifdef Vulkan
// We are compiling for SPIR-V
layout (std140) uniform ubo_model
{
    mat4 m_matrix;
} Ubo_model;
#else
// Classic GLSL
uniform mat4 u_model = mat4(0.0f);
#endif

layout (location = 0) in uint in_entity_ID;
layout (location = 1) in vec3 in_position;
layout (location = 2) in vec3 in_normal;
layout (location = 3) in vec4 in_color;
layout (location = 4) in vec2 in_tex_coords;

#ifdef Vulkan
layout (location = 0) flat out uint vout_entity_ID;
layout (location = 1) out Vertex_data_s vout_vertex_data;
#else
flat out uint vout_entity_ID;
out Vertex_data_s vout_vertex_data;
#endif

void main() {
    #ifdef Vulkan
    gl_Position = Ubo_camera.m_projection * Ubo_camera.m_view * (Ubo_model.m_matrix * vec4(in_position, 1.0));
    #else
    gl_Position = u_view_projection * (u_model * vec4(in_position, 1.0));
    #endif
    vout_entity_ID = in_entity_ID;
    vout_vertex_data.vout_normal = in_normal;
    vout_vertex_data.vout_tex_coords = in_tex_coords;
    vout_vertex_data.vout_frag_color = in_color;
}