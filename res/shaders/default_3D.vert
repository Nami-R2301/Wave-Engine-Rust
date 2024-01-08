#version 420

// Vertex attributes.
layout (location = 0) in int in_entity_ID;
layout (location = 1) in vec3 in_vertex_position;
layout (location = 2) in vec3 in_vertex_normal;
layout (location = 3) in vec4 in_color;
layout (location = 4) in vec2 in_tex_coords;

// Camera matrix.
layout (std140, binding = 0) uniform u_camera
{
    mat4 u_view;
    mat4 u_projection;
} Camera_u;

layout (std140, binding = 1) uniform u_model_matrix
{
    mat4 u_matrix;
} Model_matrix;

// Outputs.
struct Vertex_data_s
{
    vec2 vout_tex_coords;
    vec4 vout_frag_color;
    vec3 vout_normal;
    vec3 vout_frag_position;
    vec4 vout_directional_light_position;
};

layout (location = 0) flat out int vout_entity_ID;
layout (location = 1) out Vertex_data_s vout_vertex_data;


void main()
{
    gl_Position = Camera_u.u_projection * Camera_u.u_view * (Model_matrix.u_matrix *
    vec4(in_vertex_position.xyz, 1.0));
    vout_vertex_data.vout_tex_coords = in_tex_coords;
    vout_vertex_data.vout_frag_color = in_color;
    mat3 normal_matrix = mat3(inverse(transpose(Camera_u.u_view * Model_matrix.u_matrix)));
    vout_vertex_data.vout_normal = normalize(vec3(vec4(normal_matrix * normalize(in_vertex_normal), 0.0)));
    vout_vertex_data.vout_frag_position = (Model_matrix.u_matrix * vec4(in_vertex_position, 1.0)).xyz;
    vout_entity_ID = in_entity_ID;
}