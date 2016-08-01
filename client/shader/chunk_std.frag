#version 140

in vec3 x_color;
in vec3 surfaceNormal;
in float x_radius;
in vec2 x_tex_coord;

uniform sampler2D my_texture;

out vec3 color;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));

void main() {
    float diffuse = max(0.0, dot(sun, surfaceNormal));

    color = x_color;

    // TODO: Doc

    // check for border
    if (x_radius > 0.98) {
      color *= texture(my_texture, x_tex_coord).xyz * 0.75;
    } else {
      color *= texture(my_texture, x_tex_coord).xyz;
    }
    // color *= diffuse;
    // hack to make brighter
    color *= 1.3;
}
