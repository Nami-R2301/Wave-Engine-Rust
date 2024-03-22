#version 420 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

// Inputs.
layout (location = 0) flat in uint vout_vertex_entity_ID[];
layout (location = 1) in struct Frag_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
    vec3 wireframe_distances;
} vd_in[];

// Outputs.
layout (location = 0) flat out uint vout_entity_ID;
layout (location = 1) out Frag_data_s vout_geo_data;


void main() {
    for (int i = 0; i < 3; ++i) {
        vout_geo_data.vout_frag_color = vd_in[i].vout_frag_color;
        vout_geo_data.vout_normal = vd_in[i].vout_normal;
        vout_geo_data.vout_tex_coords = vd_in[i].vout_tex_coords;
        vout_entity_ID = vout_vertex_entity_ID[i];
        gl_Position = gl_in[i].gl_Position;

        /// Taken from : https://pastebin.com/G9grT2Kp
        // This is the easiest scheme I could think of. The attribute will be interpolated, so
        // all you have to do is set the ith dimension to 1.0 to get barycentric coordinates
        // specific to this triangle. The frag shader will interpolate and then you can just use
        // a threshold in the frag shader to figure out if you're close to an edge
        vout_geo_data.wireframe_distances = vec3(0.0);
        vout_geo_data.wireframe_distances[i] = 1.0;

        EmitVertex();
    }
    EndPrimitive();
}