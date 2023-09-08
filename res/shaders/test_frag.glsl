#version 330

struct Vertex_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
};

flat in uint vout_entity_ID;
in Vertex_data_s vout_vertex_data;

out vec4 fout_color;
out uint fout_entity_ID;

void main() {
    fout_color = vout_vertex_data.vout_frag_color;
    fout_entity_ID = vout_entity_ID;
}
