#version 140

in vec3 x_color;
out vec4 color;

void main() {
    color = vec4(x_color, 1.0);
    color = vec4(1.0, 1.0, 1.0, 1.0);
}
