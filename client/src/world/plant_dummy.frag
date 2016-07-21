#version 140

in vec3 x_color;
out vec4 color;

uniform vec2 offset;

void main() {
    color = vec4(x_color, 1.0);
    color = vec4(0.545, 0.27, 0.075, 1.0);
}
