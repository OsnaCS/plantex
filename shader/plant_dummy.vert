#version 140

in vec3 position;
in vec3 color;
out vec3 x_color;

uniform vec3 offset;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;

void main() {
    gl_Position = proj_matrix * view_matrix * vec4(position + offset, 1);
    x_color = color;
}
