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
layout (location = 1) in int in_texture_info;
layout (location = 2) in vec3 in_position;
layout (location = 3) in uint in_normal;  // (27 bits) + 1 bit: (|3 X 12 bits for floating point| + |1 bit for sign|).
layout (location = 4) in uint in_color;  // (32 bits): (|8 bits for alpha| + |8 bits for blue| + |8 bits for green| + |8 bits for red|).
layout (location = 5) in uint in_tex_coords;  // (26 bits): (|2 X 12 bits for floating point| + |2 bits for sign|).

layout (location = 0) flat out uint vout_entity_ID;
layout (location = 1) flat out int vout_texture_info;
layout (location = 2) out Frag_data_s vout_vertex_data;
layout (location = 6) out vec3 vout_frag_pos;

vec2 unpack_texture_coord()
{
    vec2 signs = vec2(
    (in_tex_coords & 0x0000000Fu),
    (in_tex_coords & 0x000000F0u) >> 4);

    vec2 floating_points = vec2(
    (in_tex_coords & 0xFFF00000u) >> 20,
    (in_tex_coords & 0x000FFF00u) >> 4);

    vec2 result = vec2(floating_points.r / 1000, floating_points.g / 1000);

    if (signs.r == 1)
    {
        result.r = result.r * -1;
    }

    if (signs.r == 1)
    {
        result.g = result.g * -1;
    }

    return result;
}

vec3 unpack_normal()
{
    uint signs = in_normal & 0x000000FFu;

    vec3 floating_points = vec3(
    (in_normal & 0xFF000000u) >> 24,    // 2^31 - 2^24 (1)
    (in_normal & 0x00FF0000u) >> 16,    // 2^23 - 2^16
    (in_normal & 0x0000FF00u) >> 8);   // 2^31 - 2^15 (2)

    vec3 result = vec3(floating_points.r / 1000.0, floating_points.g / 1000.0, floating_points.b / 1000.0);

    switch (signs) {
        case 1: {
            result.r = result.r * -1;
            break;
        }
        case 2: {
            result.g = result.g * -1;
            break;
        }
        case 3: {
            result.r = result.r * -1;
            result.g = result.g * -1;
            break;
        }
        case 8: {
            result.b = result.b * -1;
            break;
        }
        case 9: {
            result.r = result.r * -1;
            result.b = result.b * -1;
            break;
        }
        case 10: {
            result.g = result.g * -1;
            result.b = result.b * -1;
            break;
        }
        case 11: {
            result.r = result.r * -1;
            result.g = result.g * -1;
            result.b = result.b * -1;
            break;
        }
    }


    return result;
}

vec4 unpack_color()
{
    return vec4((in_color & 0x000000FFu) / 255.0, ((in_color & 0x0000FF00u) >> 8) / 255.0,
    ((in_color & 0x00FF0000u) >> 16) / 255.0, ((in_color & 0xFF000000u) >> 24) / 255.0);
}

void main() {
    if (in_entity_ID > 1024) {
        // Signal error.
        gl_Position = Ubo_camera.m_projection * Ubo_camera.m_view * (Ubo_model.m_matrix[in_entity_ID] * vec4(in_position, 1.0));
        vout_entity_ID = -1;

        vout_vertex_data.vout_normal = unpack_normal();
        // TODO Custom texture to signal error.
        vout_vertex_data.vout_tex_coords = unpack_texture_coord();
        vout_vertex_data.vout_frag_color = vec4(1.0, 0.0, 1.0, 1.0);  // Magenta for signaling error.
        return;
    }

    gl_Position = Ubo_camera.m_projection * Ubo_camera.m_view * (Ubo_model.m_matrix[in_entity_ID] * vec4(in_position, 1.0));
    vout_entity_ID = in_entity_ID;
    vout_texture_info = in_texture_info;
    mat3 model_matrix = mat3(transpose(inverse(Ubo_camera.m_view * Ubo_model.m_matrix[in_entity_ID])));
    vec3 normal = unpack_normal();
    vout_vertex_data.vout_normal = normalize(vec3(vec4(model_matrix * normalize(normal), 0.0)));
    vout_vertex_data.vout_tex_coords = unpack_texture_coord();
    vout_vertex_data.vout_frag_color = unpack_color();
    vout_frag_pos = vec3(Ubo_model.m_matrix[in_entity_ID] * vec4(in_position, 1.0));
}