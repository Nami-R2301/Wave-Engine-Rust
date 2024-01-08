#version 420

struct Vertex_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
};

layout (location = 0) flat in uint vout_entity_ID;
layout (location = 1) in Vertex_data_s vout_vertex_data;

layout (location = 0) out vec4 fout_color;
layout (location = 1) out uint fout_entity_ID;

void main() {
    fout_color = vout_vertex_data.vout_frag_color;
    fout_entity_ID = vout_entity_ID;
}
