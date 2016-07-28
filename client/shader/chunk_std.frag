#version 140

in vec3 x_color;
in vec3 surfaceNormal;
in vec2 x_tex_coord;

uniform sampler2D my_texture;

out vec3 color;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));
void main() {
    float diffuse = max(0.0, dot(sun, surfaceNormal));
    color = x_color * 0.1 + x_color * diffuse;

    float hexagon_width = sqrt(3.0)*0.5;

    float step_x = 4.0;
    float step_y = step_x / hexagon_width;

    if (((mod(step_x * x_tex_coord.x, 1.0) < 0.5) || (mod(step_y * x_tex_coord.y, 1.0) < 0.5))

        && (!((mod(step_x * x_tex_coord.x, 1.0) < 0.5) && (mod(step_y * x_tex_coord.y, 1.0) < 0.5)))) {
        color = vec3(1.0, 1.0, 1.0);
    } else {
        color = vec3(0.0, 0.0, 0.0);
    }

    if (x_tex_coord.x == -1.0 && x_tex_coord.y == -1.0) {
        color = vec3(0.0, 1.0, 1.0);
    }

    // color = vec3(x_tex_coord.x);
    // color = texture(my_texture, x_tex_coord).xyz;
}
