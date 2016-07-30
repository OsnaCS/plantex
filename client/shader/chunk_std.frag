#version 140

in vec4 shadowCoord;
in vec3 x_color;
in vec3 surfaceNormal;
in float x_radius;
in vec2 x_tex_coord;
flat in int x_ground;

uniform sampler2D sand_texture;
uniform sampler2D grass_texture;
uniform sampler2D snow_texture;

out vec3 color;

// FIXME This should be a `sampler2DShadow`, but glium doesn't expose it
uniform sampler2D shadow_map;
<<<<<<< a2ef8fabfc8560d50448076b951bccdc2e776d39
const float SHADOW_BIAS = 0.005;    // Prevent "acne" :D
=======
// Shadow map height/width in pixels:
// TODO Set from Rust code so it's always consistent
uniform float shadow_map_size = 1024;
// Percentage-closer filtering (square) radius in pixels
const int SHADOW_PCF_RADIUS = 2;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));
const float SHADOW_BIAS = 0.00001;    // FIXME does this even work?
>>>>>>> PCF shadows
const float AMBIENT = 0.1;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));

void main() {
    float diffuse = max(0.0, dot(sun, surfaceNormal));

    color = x_color * AMBIENT + x_color * diffuse;


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

    // Divide by number of pixels we sampled, then by 2 to get it into a range
    // from 0 to 0.5
    shadowFactor /= PCF_PIXELS * PCF_PIXELS * 2;

    // FIXME Be smarter about this calculation - We simply make the whole color
    // darker
    color = color * (0.7 - shadowFactor);
    // TODO: More grounds and make it better ;D
    if(x_ground == 1) {
        if (x_radius > 0.98) {
            color *= texture(grass_texture, x_tex_coord).xyz * 0.75;
        } else {
            color *= texture(grass_texture, x_tex_coord).xyz;
        }
    } else if(x_ground == 2) {
        if (x_radius > 0.98) {
          color *= texture(sand_texture, x_tex_coord).xyz * 0.75;
        } else {
          color *= texture(sand_texture, x_tex_coord).xyz;
        }
    } else {
        if (x_radius > 0.98) {
          color *= texture(snow_texture, x_tex_coord).xyz * 0.75;
        } else {
          color *= texture(snow_texture, x_tex_coord).xyz;
        }
    }
    // check for border
    // if (x_radius > 0.98) {
    //   color *= texture(my_texture, x_tex_coord).xyz * 0.75;
    // } else {
    //   color *= texture(my_texture, x_tex_coord).xyz;
    // }
    // color *= diffuse;
    // hack to make brighter
}
