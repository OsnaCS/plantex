#version 140

in vec3 position;
in vec3 xyz;

out float pre_theta;
out vec2 pre_phi;

out vec3 unit_coords;

uniform mat4 proj_matrix;
uniform mat4 view_matrix;

void main() {

    gl_Position = proj_matrix * view_matrix * vec4(position, 1);

    pre_theta = xyz.z;
    pre_phi = vec2(xyz.x, xyz.y);

    unit_coords = xyz;
}
