#version 150

in vec3 i_position;
in vec3 i_unit_coords;

uniform mat4 u_proj_matrix;
uniform mat4 u_view_matrix;

out vec3 x_unit_coords;

void main() {
gl_Position = u_proj_matrix * u_view_matrix * vec4(i_position, 1);
x_unit_coords = i_unit_coords;
}
