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
    color = x_color * 0.1 + x_color * diffuse;

    float n = 16.0;

    // n * hexagon width
    float steps = n * sqrt(3.0)*0.5;

    // ------------------------------------------------
    // Checkered black and white texture -- leave this!
    // ------------------------------------------------
    // if ((mod(steps * (x_tex_coord.x), 1.0) < 0.5) != (mod(steps * x_tex_coord.y, 1.0) < 0.5)) {
    //     color = vec3(1.0, 1.0, 1.0);
    // } else {
    //     color = vec3(0.0, 0.0, 0.0);
    // }

    // TODO: Doc
    if (x_tex_coord.x == -1.0 && x_tex_coord.y == -1.0) {
        color = vec3(0.0, 0.0, 0.0);
    } else {
        if (x_radius > 0.98) {
          color = texture(my_texture, x_tex_coord).xyz * 0.3;
        } else {
          color = texture(my_texture, x_tex_coord).xyz;
        }

    }
}
