#version 420 core

struct Vertex_data_s
{
    vec2 vout_tex_coords;// Texture coordinates.
    vec4 vout_frag_color;// Base material color.
    vec3 vout_normal;
    vec3 vout_frag_position;// Start of fragment position.
    vec4 vout_directional_light_position;// Position of the light source.
};

struct Light
{
    vec4 color;
    float ambient_intensity;
    float diffuse_intensity;
};

struct DirectionalLight
{
    Light base;
    vec3 direction;
};

struct PointLight
{
    Light base;
    vec3 position;
    float constant;
    float linear;
    float exponent;
};

struct SpotLight
{
    PointLight base;
    vec3 direction;
    float edge;
};

struct Material
{
    float specularIntensity;
    float shininess;
};

layout (location = 0) flat in int vout_entity_ID;
layout (location = 1) in Vertex_data_s vout_vertex_data;

layout (location = 0) out vec4 fout_frag_color;
layout (location = 1) out int fout_entity_ID;

// Light count.
uniform int u_point_light_count;
uniform int u_spot_light_count;
const int MAX_POINT_LIGHTS = 5;
const int MAX_SPOT_LIGHTS = 5;

// Texture.
layout (binding = 0) uniform sampler2D u_sampler;
uniform bool u_has_texture = false;
uniform bool u_affected_by_light = true;

// Different light structures.
uniform Material material;
uniform DirectionalLight u_directional_light;
uniform PointLight u_point_lights[MAX_POINT_LIGHTS];
uniform SpotLight u_spot_lights[MAX_SPOT_LIGHTS];

uniform vec3 u_eye_position;

vec4 CalcLightByDirection(Light light, vec3 direction)
{
    vec4 ambientColour = light.color * light.ambient_intensity;

    float diffuseFactor = max(dot(normalize(vout_vertex_data.vout_normal), normalize(direction)), 0.0f);
    vec4 diffuseColour = light.color * light.diffuse_intensity * diffuseFactor;

    vec4 specularColour = vec4(0, 0, 0, 0);

    if (diffuseFactor > 0.0f)
    {
        vec3 fragToEye = normalize(u_eye_position - vout_vertex_data.vout_frag_position);
        vec3 reflectedVertex = normalize(reflect(direction, normalize(vout_vertex_data.vout_normal)));

        float specularFactor = dot(fragToEye, reflectedVertex);
        if (specularFactor > 0.0f)
        {
            specularFactor = pow(specularFactor, material.shininess);
            specularColour = light.color * material.specularIntensity * specularFactor;
        }
    }

    return (ambientColour * (diffuseColour + specularColour));
}

vec4 CalcDirectionalLight()
{
    return CalcLightByDirection(u_directional_light.base, u_directional_light.direction);
}

vec4 CalcPointLight(PointLight pLight)
{
    vec3 direction = vout_vertex_data.vout_frag_position - pLight.position;
    float distance = length(direction);
    direction = normalize(direction);

    vec4 color = CalcLightByDirection(pLight.base, direction);
    float attenuation = pLight.exponent * distance * distance + pLight.linear * distance + pLight.constant;

    return (color / attenuation);
}

vec4 CalcSpotLight(SpotLight sLight)
{
    vec3 rayDirection = normalize(vout_vertex_data.vout_frag_position - sLight.base.position);
    float slFactor = dot(rayDirection, sLight.direction);

    if (slFactor > sLight.edge)
    {
        vec4 colour = CalcPointLight(sLight.base);
        return colour * (1.0f - (1.0f - slFactor) * (1.0f / (1.0f - sLight.edge)));

    } else return vec4(0, 0, 0, 0);
}

vec4 CalcPointLights()
{
    vec4 totalColour = vec4(0, 0, 0, 0);
    for (int i = 0; i < u_point_light_count; i++)
    {
        totalColour += CalcPointLight(u_point_lights[i]);
    }
    return totalColour;
}

vec4 CalcSpotLights()
{
    vec4 totalColour = vec4(0, 0, 0, 0);
    for (int i = 0; i < u_spot_light_count; i++)
    {
        totalColour += CalcSpotLight(u_spot_lights[i]);
    }
    return totalColour;
}

void main()
{
    vec4 material_color = vec4(1.0f);
    if (!u_has_texture) material_color *= vout_vertex_data.vout_frag_color;
    else material_color = texture(u_sampler, vout_vertex_data.vout_tex_coords);

    vec4 light_result = vec4(1.0f);
    if (u_affected_by_light)
    {

        //        light_result = CalcDirectionalLight();
        //                light_result = CalcPointLights();
        //                light_result *= vout_vertex_data.vout_frag_color;
        //        light_result += CalcSpotLights();
        //        light_result *= vout_frag_color;

        vec3 norm = normalize(vout_vertex_data.vout_normal);
        vec3 lightDir = normalize(vec3(u_point_lights[0].position - vout_vertex_data.vout_frag_position));
        vec3 reflectDir = reflect(-lightDir, norm);
        float diffuse_strength = max(0.0f, dot(norm, lightDir));
        float specular_strength = pow(max(0.0f, dot(u_point_lights[0].position, reflectDir)), 32);

        vec3 ambient = u_point_lights[0].base.ambient_intensity * vout_vertex_data.vout_frag_color.xyz;
        vec3 specular = specular_strength * vout_vertex_data.vout_frag_color.xyz;
        vec3 diffuse = diffuse_strength * u_point_lights[0].base.diffuse_intensity * vout_vertex_data.vout_frag_color.xyz;

        light_result = vec4((ambient + diffuse + specular), vout_vertex_data.vout_frag_color.w);
    }
    fout_frag_color = material_color * light_result;
    fout_entity_ID = vout_entity_ID;
}