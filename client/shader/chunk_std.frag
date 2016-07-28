#version 140

in vec4 shadowCoord;
in vec3 x_color;
in vec3 surfaceNormal;

out vec3 color;

uniform sampler2D shadow_map;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));
const float SHADOW_BIAS = 0.005;    // Prevent "acne" :D
const float AMBIENT = 0.1;

void main() {
    float diffuse = max(0.0, dot(sun, surfaceNormal));
    color = x_color * AMBIENT + x_color * diffuse;

    vec3 lightCoords = shadowCoord.xyz / shadowCoord.w;
    lightCoords = lightCoords * 0.5 + 0.5;
    if (texture(shadow_map, lightCoords.xy).r < lightCoords.z - SHADOW_BIAS) {
        // Something is between this fragment and the sun => Shadow
        color *= 0.5;
    }
}
