#version 140

in vec4 shadowCoord;
in vec3 x_color;
in vec3 surfaceNormal;

in float x_radius;
in vec2 x_tex_coords;



out vec3 color;

// Vector from the camera to the sun
uniform vec3 sun_dir;
// FIXME This should be a `sampler2DShadow`, but glium doesn't expose it
uniform sampler2D shadow_map;

uniform sampler2D my_texture;
uniform sampler2D normals;

// Percentage-closer filtering (square) radius in pixels
const int SHADOW_PCF_RADIUS = 1;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));
const float SHADOW_BIAS = 0.00001;    // FIXME does this even work?
const float AMBIENT = 0.2;

void main() {
    // Shadow map height/width in pixels:
    float SHADOW_MAP_SIZE = textureSize(shadow_map, 0).x;

    vec3 lightCoords = shadowCoord.xyz / shadowCoord.w;
    lightCoords = lightCoords * 0.5 + 0.5;
    float pixelOffset = 1.0 / SHADOW_MAP_SIZE;
    float shadowFactor = 0.0;

    for (int y = -SHADOW_PCF_RADIUS; y <= SHADOW_PCF_RADIUS; y++) {
        for (int x = -SHADOW_PCF_RADIUS; x <= SHADOW_PCF_RADIUS; x++) {
            vec2 offset = vec2(x * pixelOffset, y * pixelOffset);
            vec2 mapCoords = lightCoords.xy + offset;
            if (mapCoords.x > 1.0 || mapCoords.x < 0 || mapCoords.y > 1 || mapCoords.y < 0) {
                // Guess the shadow depending on the sun angle
                float sunVertDot = 0.5 + dot(sun_dir, vec3(0, 0, 1)) * 0.5;
                shadowFactor += sunVertDot * 0.8;
            } else {
                bool shadow = texture(shadow_map, mapCoords).r
                    < lightCoords.z + SHADOW_BIAS;
                shadowFactor += shadow ? 1.0 : 0.0;
            }
        }
    }

    // Height/Width of the square we sample for Percentage Closer Filtering
    // (in Pixels)
    const int PCF_PIXELS = 1 + 2 * SHADOW_PCF_RADIUS;

    // Divide by number of pixels we sampled, to get  a range from 0 to 1
    shadowFactor /= PCF_PIXELS * PCF_PIXELS;

    // Do the normal light calculation. Ambient light is not affected by shadow,
    // other lights are coming from the sun so they're affected.
    float diffuse = max(0.0, dot(-sun_dir, surfaceNormal));
    color = x_color * AMBIENT + x_color * diffuse * (1.0 - shadowFactor);

    if (x_radius > 0.98) {
        color = texture(normals, x_tex_coords).rgb;
    } else {
        color = texture(normals, x_tex_coords).rgb;
    }

    // color = texture(normals, x_tex_coords).rgb;

    // BORDERS
    // check for border
    // if (x_radius > 0.98) {
    //   color *= texture(normals, x_tex_coordx).xyz * 0.75;
    // } else {
    //   color *= texture(normals, x_tex_coordx).xyz;
    // }

}
