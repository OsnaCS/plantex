#version 140

in vec3 x_color;
in vec3 surfaceNormal;
in float x_radius;
in vec2 x_tex_coord;
flat in int x_ground;

uniform sampler2D sand_texture;
uniform sampler2D grass_texture;
uniform sampler2D snow_texture;

out vec3 color;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));

void main() {
    float diffuse = max(0.0, dot(sun, surfaceNormal));

    color = x_color;

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
    color *= 1.3;
}
