#version 140

in vec3 position;
in vec3 color;
out vec3 x_color;

void main() {
    gl_Position = vec4(position, 1);
    x_color = color;
}
