#version 420 core

struct Frag_data_s
{
    vec3 vout_normal;
    vec4 vout_frag_color;
    vec2 vout_tex_coords;
    vec3 wireframe_distances;
};

layout (location = 0) flat in uint vout_entity_ID;
layout (location = 1) flat in int vout_texture_info;    // (31 bits + sign): |11 bits -> texture size| + |10 bits texture depth end index| +
                                                        // |10 bits texture depth start|
layout (location = 2) in Frag_data_s vout_vertex_data;
layout (location = 6) in vec3 vout_frag_pos;

// Texture array buckets.
layout (binding = 3) uniform sampler2DArray s_texture_array_64;
layout (binding = 4) uniform sampler2DArray s_texture_array_128;
layout (binding = 5) uniform sampler2DArray s_texture_array_256;
layout (binding = 6) uniform sampler2DArray s_texture_array_512;
layout (binding = 7) uniform sampler2DArray s_texture_array_1024;
layout (binding = 8) uniform sampler2DArray s_texture_array_2048;

layout (std140, binding = 9) uniform ubo_wireframe
{
    bool is_enabled[1024];
} Ubo_wireframe;

layout (std140, binding = 10) uniform ubo_light
{
    bool is_enabled;
    vec3 position;
} Ubo_light;

layout (location = 0) out vec4 fout_color;
layout (location = 1) out uint fout_entity_ID;

void main() {
    vec4 texture_color = vec4(1.0);

    if (vout_texture_info >= 0)
    {
        // Check where the texture depth starts in the array and where it ends in case of multiple textures on top of each other.
        uvec2 depth_bounds = uvec2(
        (vout_texture_info & 0x000000FF),
        (vout_texture_info & 0x0000FF00) >> 8);
        uint size = (vout_texture_info & 0x07FF0000) >> 16;

        // Mix textures if there is more than one texture assigned to the current triangle.
        for (uint i = depth_bounds.r; i < depth_bounds.g; ++i)
        {
            // Read the texture from the appropriate 'bucket' to ensure size uniformality.
            switch (size) {
                case 64: {
                    texture_color = vec4(texture(s_texture_array_64, vec3(vout_vertex_data.vout_tex_coords, i)).rgb, 1.0);
                    break;
                }
                case 128: {
                    texture_color = vec4(texture(s_texture_array_128, vec3(vout_vertex_data.vout_tex_coords, i)).rgb, 1.0);
                    break;
                }
                case 256: {
                    texture_color = vec4(texture(s_texture_array_256, vec3(vout_vertex_data.vout_tex_coords, i)).rgb, 1.0);
                    break;
                }
                case 512: {
                    texture_color = vec4(texture(s_texture_array_512, vec3(vout_vertex_data.vout_tex_coords, i)).rgb, 1.0);
                    break;
                }
                case 1024: {
                    texture_color = vec4(texture(s_texture_array_1024, vec3(vout_vertex_data.vout_tex_coords, i)).rgb, 1.0);
                    break;
                }
                case 2048: {
                    texture_color = vec4(texture(s_texture_array_2048, vec3(vout_vertex_data.vout_tex_coords, i)).rgb, 1.0);
                    break;
                }
                default: texture_color = vec4(texture(s_texture_array_2048, vec3(vout_vertex_data.vout_tex_coords, i)).rgb, 1.0);
            }
        }
        if (texture_color == vec4(0.0, 0.0, 0.0, 1.0)) {
            texture_color = vec4(1.0, 0.0, 1.0, 1.0);  // Magenta to signal error.
        }
    }

    // Lighting calculations.
    const vec3 light_color = vec3(1.0, 1.0, 1.0);
    const float ambient_strength = 0.05;
    const vec3 ambient = ambient_strength * light_color;

    const vec3 light_pos = vec3(-10.0, 10.0, -5.0);
    const vec3 norm = normalize(vout_vertex_data.vout_normal);
    const vec3 light_dir = normalize(light_pos - vout_frag_pos);

    const float diff = max(dot(norm, light_dir), 0.0);
    const vec3 diffuse = diff * light_color;

    const float pi = 3.14159265;
    const float shininess = 16.0;
    const float energy_conservation = (2.0 + shininess) / (2.0 * pi);

    const float specular_strength = diffuse != vec3(0.0) ? 0.5 : 0.0;
    const vec3 view_dir = normalize(vec3(0.0) - vout_frag_pos);

    vec3 reflect_dir = reflect(-light_dir, vout_vertex_data.vout_normal);
    float spec = energy_conservation * pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    vec3 specular = specular_strength * spec * light_color;

    vec3 result = (ambient + diffuse + specular) * vout_vertex_data.vout_frag_color.rgb;

    // Fragment shader snippet
    // The procedure here is directly adapted from the example at
    // http://codeflow.org/entries/2012/aug/02/easy-wireframe-display-with-barycentric-coordinates/
    if (Ubo_wireframe.is_enabled[vout_entity_ID])
    {
        vec3 line_width = fwidth(vout_vertex_data.wireframe_distances);

        vec3 aa_strength = smoothstep(vec3(0.0), line_width * 1.3, vout_vertex_data.wireframe_distances);
        float edgeFactor = min(min(aa_strength.x, aa_strength.y), aa_strength.z);

        fout_color = vec4(mix(vec3(0.0), result.rgb, edgeFactor), vout_vertex_data.vout_frag_color.a) * texture_color;
        //        fout_color = vec4(mix(fout_color.rgb, vec3(0.0), edgeFactor), mix(0.0, vout_vertex_data.vout_frag_color.a, edgeFactor));
        //        fout_color = vec4(mix(fout_color.rgb, vec3(0.0), edgeFactor), (1.0 - fout_color) * 0.95);
        fout_entity_ID = vout_entity_ID;
        return;
    }
//    fout_color = vout_vertex_data.vout_frag_color * texture_color;
    fout_color = vec4(result, vout_vertex_data.vout_frag_color.a) * texture_color;
    fout_entity_ID = vout_entity_ID;
}
