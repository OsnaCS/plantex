#version 140

in vec4 shadowCoord;
in vec3 x_color;
in vec3 surfaceNormal;

out vec3 color;

// FIXME This should be a `sampler2DShadow`, but glium doesn't expose it
uniform sampler2D shadow_map;
// Shadow map height/width in pixels:
// TODO Set from Rust code so it's always consistent
uniform float shadow_map_size = 1024;
// Percentage-closer filtering (square) radius in pixels
const int SHADOW_PCF_RADIUS = 2;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));
const float SHADOW_BIAS = 0.00001;    // FIXME does this even work?
const float AMBIENT = 0.2;

void main() {
    vec3 lightCoords = shadowCoord.xyz / shadowCoord.w;
    lightCoords = lightCoords * 0.5 + 0.5;
    float pixelOffset = 1.0 / shadow_map_size;
    float shadowFactor = 0.0;

    for (int y = -SHADOW_PCF_RADIUS; y <= SHADOW_PCF_RADIUS; y++) {
        for (int x = -SHADOW_PCF_RADIUS; x <= SHADOW_PCF_RADIUS; x++) {
            vec2 offset = vec2(x * pixelOffset, y * pixelOffset);
            bool shadow = texture(shadow_map, lightCoords.xy + offset).r
                < lightCoords.z + SHADOW_BIAS;
            shadowFactor += shadow ? 1.0 : 0.0;
        }
    }

    // Height/Width of the square we sample for Percentage Closer Filtering
    // (in Pixels)
    const int PCF_PIXELS = 1 + 2 * SHADOW_PCF_RADIUS;

    // Divide by number of pixels we sampled, to get  a range from 0 to 1
    shadowFactor /= PCF_PIXELS * PCF_PIXELS;

    // Do the normal light calculation. Ambient light is not affected by shadow,
    // other lights are coming from the sun so they're affected.
    float diffuse = max(0.0, dot(sun, surfaceNormal));
    color = x_color * AMBIENT + x_color * diffuse * (1.0 - shadowFactor);
}
