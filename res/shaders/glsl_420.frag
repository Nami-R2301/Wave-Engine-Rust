#version 420 core

struct Frag_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
    vec3 wireframe_distances;
};

layout (location = 0) flat in uint vout_entity_ID;
layout (location = 1) in Frag_data_s vout_vertex_data;

layout (location = 0) out vec4 fout_color;
layout (location = 1) out uint fout_entity_ID;

layout (std140, binding = 3) uniform ubo_wireframe
{
    bool is_enabled;
} Ubo_wireframe;

void main() {
    // Fragment shader snippet
    // The procedure here is directly adapted from the example at
    // http://codeflow.org/entries/2012/aug/02/easy-wireframe-display-with-barycentric-coordinates/
    if (Ubo_wireframe.is_enabled)
    {
        vec3 line_width = fwidth(vout_vertex_data.wireframe_distances);

        vec3 aa_strength = smoothstep(vec3(0.0), line_width * 1.3, vout_vertex_data.wireframe_distances);
        float edgeFactor = min(min(aa_strength.x, aa_strength.y), aa_strength.z);

        fout_color = vec4(mix(vec3(0.0), vout_vertex_data.vout_frag_color.rgb, edgeFactor),
        vout_vertex_data.vout_frag_color.a);
//        fout_color = vec4(mix(fout_color.rgb, vec3(0.0), edgeFactor), mix(0.0, vout_vertex_data.vout_frag_color.a, edgeFactor));
//        fout_color = vec4(mix(fout_color.rgb, vec3(0.0), edgeFactor), (1.0 - fout_color) * 0.95);
        fout_entity_ID = vout_entity_ID;
        return;
    }
    fout_color = vout_vertex_data.vout_frag_color;
    fout_entity_ID = vout_entity_ID;
}
