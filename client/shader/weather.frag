#version 330

in vec2 out_position;
in vec4 out_color;

uniform int form;
uniform vec3 sky_light;
uniform vec3 sun_color;

out vec4 color;

void main() {

    // multiply our color with given HDR values
    color = vec4(out_color) * vec4(sky_light, 1.0) * vec4(sun_color, 1.0);

    if (length(out_position) > 1) {
        discard;
    }

    if (form == 3) {
        color.a = 0.7 - (length(out_position)/2.0);
    }

    if (form == 2) {
        color.a = 1 - (length(out_position)/1.5);
    }
}
