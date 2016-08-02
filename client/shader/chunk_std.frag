#version 140

in vec4 shadowCoord;
in vec3 x_color;
in vec3 surfaceNormal;

out vec3 color;

// Vector from the camera to the sun
uniform vec3 sun_dir;
uniform sampler2D shadow_map;
// Percentage-closer filtering (square) radius in pixels
const int SHADOW_PCF_RADIUS = 1;

const float SHADOW_BIAS = 0.001;    // FIXME does this even work?
const float AMBIENT = 0.2;

float lightCoverage(vec2 moments, float fragDepth) {
    float E_x2 = moments.y;
    float Ex_2 = moments.x * moments.x;
    float variance = E_x2 - Ex_2;
    float mD = moments.x - fragDepth;
    float mD_2 = mD * mD;
    float p = variance / (variance + mD_2);
    return min(max(p, fragDepth <= moments.x ? 1.0 : 0.0), 1.0);
}

void main() {
    vec3 lightCoords = shadowCoord.xyz / shadowCoord.w;
    lightCoords = lightCoords * 0.5 + 0.5;
    vec2 moments = texture(shadow_map, lightCoords.xy).xy;
    float lit = max(lightCoverage(moments, lightCoords.z - SHADOW_BIAS), 0.2);

    // Do the normal light calculation. Ambient light is not affected by shadow,
    // other lights are coming from the sun so they're affected.
    float diffuse = max(0.0, dot(-sun_dir, surfaceNormal));
    color = x_color * AMBIENT + x_color * diffuse * lit;
}
