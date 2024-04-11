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
layout (location = 6) in vec3 vout_frag_pos;

layout (location = 0) out vec4 fout_color;
layout (location = 1) out uint fout_entity_ID;

layout (std140, binding = 3) uniform ubo_wireframe
{
    bool is_enabled;
} Ubo_wireframe;

layout (std140, binding = 4) uniform ubo_light
{
    bool is_enabled;
    vec3 position;
} Ubo_light;

void main() {
    // Lighting calculations.
    vec3 light_color = vec3(1.0);
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;

    vec3 light_pos = vec3(-0.5, 0.5, 0.0);
    vec3 norm = normalize(vout_vertex_data.vout_normal);
    vec3 light_dir = normalize(light_pos - vout_frag_pos);

    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_color;

    const float kPi = 3.14159265;
    const float kShininess = 16.0;

    float specular_strength = diffuse != vec3(0.0) ? 0.5 : 0.0;
    float shininess = 16.0;
    vec3 view_dir = normalize(vec3(0.0) - vout_frag_pos);

    const float kEnergyConservation = ( 2.0 + kShininess ) / ( 2.0 * kPi );
    vec3 reflect_dir = reflect(-light_dir, vout_vertex_data.vout_normal);
    float spec = kEnergyConservation * pow(max(dot(view_dir, reflect_dir), 0.0), kShininess);

//    float spec = pow(max(dot(view_dir, halfway_dir), 0.0), shininess);
    vec3 specular = specular_strength * spec * light_color;

    vec3 result = (ambient + diffuse + specular) * vout_vertex_data.vout_frag_color.rgb;

    // Fragment shader snippet
    // The procedure here is directly adapted from the example at
    // http://codeflow.org/entries/2012/aug/02/easy-wireframe-display-with-barycentric-coordinates/
    if (Ubo_wireframe.is_enabled)
    {
        vec3 line_width = fwidth(vout_vertex_data.wireframe_distances);

        vec3 aa_strength = smoothstep(vec3(0.0), line_width * 1.3, vout_vertex_data.wireframe_distances);
        float edgeFactor = min(min(aa_strength.x, aa_strength.y), aa_strength.z);

        fout_color = vec4(mix(vec3(0.0), vout_vertex_data.vout_frag_color.rgb * result, edgeFactor),
        vout_vertex_data.vout_frag_color.a);
        //        fout_color = vec4(mix(fout_color.rgb, vec3(0.0), edgeFactor), mix(0.0, vout_vertex_data.vout_frag_color.a, edgeFactor));
        //        fout_color = vec4(mix(fout_color.rgb, vec3(0.0), edgeFactor), (1.0 - fout_color) * 0.95);
        fout_entity_ID = vout_entity_ID;
        return;
    }

    fout_color = vec4(result, 1.0);
    fout_entity_ID = vout_entity_ID;
}
