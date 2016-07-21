#version 140

in vec3 position;
in vec3 color;
out vec3 x_color;

uniform vec2 offset;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;

void main() {
    gl_Position = proj_matrix * view_matrix * vec4(position.x + offset[0],
        position.y + offset[1], position.z, 1);
    x_color = color;
}
