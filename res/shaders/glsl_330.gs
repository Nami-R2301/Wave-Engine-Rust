#version 330 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

// Inputs.
layout (location = 0) flat in uint vout_vertex_entity_ID[];
layout (location = 1) in struct Frag_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
    vec3 vout_wireframe_distances;
} vd_in[];

// Outputs.
flat out uint gout_entity_ID;
out Frag_data_s gout_geo_data;


void main() {
    for (int i = 0; i < 3; ++i) {
        gout_geo_data.vout_frag_color = vd_in[i].vout_frag_color;
        gout_geo_data.vout_normal = vd_in[i].vout_normal;
        gout_geo_data.vout_tex_coords = vd_in[i].vout_tex_coords;
        gout_entity_ID = vout_vertex_entity_ID[i];
        gl_Position = gl_in[i].gl_Position;

        /// Taken from : https://pastebin.com/G9grT2Kp
        // This is the easiest scheme I could think of. The attribute will be interpolated, so
        // all you have to do is set the ith dimension to 1.0 to get barycentric coordinates
        // specific to this triangle. The frag shader will interpolate and then you can just use
        // a threshold in the frag shader to figure out if you're close to an edge
        gout_geo_data.vout_wireframe_distances = vec3(0.0);
        gout_geo_data.vout_wireframe_distances[i] = 1.0;

        EmitVertex();
    }
    EndPrimitive();
}