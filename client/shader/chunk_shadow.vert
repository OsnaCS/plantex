#version 140

// Per-vertex attributes
in vec3 position;
in vec3 normal;

uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform vec2 offset;

void main() {
    vec4 world_coords = vec4(
        position.xy + offset.xy,
        position.z,
        1);
    gl_Position = proj_matrix * view_matrix * world_coords;
}
