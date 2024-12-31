#version 330 core

struct Frag_data_s
{
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
};

layout (location = 0) in uint in_entity_ID;
layout (location = 1) in vec3 in_position;
layout (location = 3) in uint in_color;
layout (location = 4) in vec2 in_tex_coords;

layout (std140, binding = 1) uniform ubo_model
{
    mat4 m_matrix[255];  // Minimum array count for uniforms (taking into account vec4s).
} Ubo_model;

layout (location = 0) flat out uint vout_entity_ID;
layout (location = 1) flat out int vout_texture_info;
layout (location = 2) out Frag_data_s vout_vertex_data;
layout (location = 5) out vec3 vout_frag_pos;

void main() {
    if (in_entity_ID > 255) {
        // Signal error.
        gl_Position = (Ubo_model.m_matrix[254] * vec4(in_position, 1.0));
        vout_entity_ID = -1;
        // TODO Custom texture to signal error.
        vout_vertex_data.vout_tex_coords = in_tex_coords;
        vout_vertex_data.vout_frag_color = vec4(1.0, 0.0, 1.0, 1.0);
        vout_frag_pos = vec3(Ubo_model.m_matrix[254] * vec4(in_position, 1.0));
        return;
    }
    gl_Position = (Ubo_model.m_matrix[in_entity_ID] * vec4(in_position, 1.0));
    vout_entity_ID = in_entity_ID;
    vout_vertex_data.vout_tex_coords = in_tex_coords;
    vout_vertex_data.vout_frag_color = vec4((in_color & 0x000000FFu) / 255.0, ((in_color & 0x0000FF00u) >> 8) / 255.0,
    ((in_color & 0x00FF0000u) >> 16) / 255.0, ((in_color & 0xFF000000u) >> 24) / 255.0);
    vout_frag_pos = vec3(Ubo_model.m_matrix[in_entity_ID] * vec4(in_position, 1.0));
}
