#version 140

in vec3 x_color;
out vec3 color;

uniform vec2 offset;

void main() {
    color = x_color;
}
