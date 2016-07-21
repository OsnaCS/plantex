#version 140

in vec3 x_color;
out vec4 color;

uniform vec2 offset;
uniform mat4 scale_matrix;

void main() {
    color = vec4(x_color, 1.0);
}
