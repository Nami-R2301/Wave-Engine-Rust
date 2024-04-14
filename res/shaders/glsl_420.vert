#version 420 core

// Outputs.
struct Frag_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
    vec3 wireframe_distances;
};

layout (std140, binding = 0) uniform ubo_camera
{
    mat4 m_view;
    mat4 m_projection;
} Ubo_camera;


layout (std140, binding = 1) uniform ubo_model
{
    mat4 m_matrix[1024];  // Minimum array count for uniforms (taking into account vec4s).
} Ubo_model;

layout (location = 0) in uint in_entity_ID;
layout (location = 1) in vec3 in_position;
layout (location = 2) in vec3 in_normal;
layout (location = 3) in uint in_color;
layout (location = 4) in vec2 in_tex_coords;

layout (location = 0) flat out uint vout_entity_ID;
layout (location = 1) out Frag_data_s vout_vertex_data;
layout (location = 5) out vec3 vout_frag_pos;

void main() {
    if (in_entity_ID > 1024) {
        // Signal error.
        gl_Position = Ubo_camera.m_projection * Ubo_camera.m_view * (Ubo_model.m_matrix[in_entity_ID] * vec4(in_position, 1.0));
        vout_entity_ID = -1;
        vout_vertex_data.vout_normal = in_normal;
        // TODO Custom texture to signal error.
        vout_vertex_data.vout_tex_coords = in_tex_coords;
        vout_vertex_data.vout_frag_color = vec4(1.0, 0.0, 1.0, 1.0);
        return;
    }
    gl_Position = Ubo_camera.m_projection * Ubo_camera.m_view * (Ubo_model.m_matrix[in_entity_ID] * vec4(in_position, 1.0));
    vout_entity_ID = in_entity_ID;
    mat3 normal_matrix = mat3(transpose(inverse(Ubo_camera.m_view * Ubo_model.m_matrix[in_entity_ID])));
    vout_vertex_data.vout_normal = normalize(vec3(vec4(normal_matrix * normalize(in_normal), 0.0)));
    vout_vertex_data.vout_tex_coords = in_tex_coords;
    vout_vertex_data.vout_frag_color = vec4((in_color & 0x000000FFu) / 255.0, ((in_color & 0x0000FF00u) >> 8) / 255.0,
    ((in_color & 0x00FF0000u) >> 16) / 255.0, ((in_color & 0xFF000000u) >> 24) / 255.0);
    vout_frag_pos = vec3(Ubo_model.m_matrix[in_entity_ID] * vec4(in_position, 1.0));
}