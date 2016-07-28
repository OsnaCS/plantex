#version 150

in vec3 i_position;
in vec3 i_unit_coords;

uniform mat4 u_proj_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_model;

out vec3 x_unit_coords;

void main() {

    mat4 matrix = u_view_matrix * u_model;
    matrix[0][0] = 1.0;
    matrix[0][1] = 0.0;
    matrix[0][2] = 0.0;

    matrix[1][0] = 0.0;
    matrix[1][1] = 1.0;
    matrix[1][2] = 0.0;

    matrix[2][0] = 0.0;
    matrix[2][1] = 0.0;
    matrix[2][2] = 1.0;

    gl_Position = u_proj_matrix * matrix * vec4(i_position, 1);
    x_unit_coords = i_unit_coords;
}
