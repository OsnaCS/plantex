#version 140

// Per-vertex attributes
in vec3 position;
in vec3 normal;

// Per-instance attributes:
// Height in units, not world coordinates, since the "pillar prototype" has a
// height of one unit.
in float height;
in vec3 offset;
in vec3 material_color;

out vec3 x_color;
out vec3 surfaceNormal;
// Vertex/Pixel coordinates in shadow map
out vec4 shadowCoordT;

uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform mat4 depth_view_proj;

void main() {
    vec4 world_coords = vec4(
        position.xy + offset.xy,
        position.z * height + offset.z,
        1);
    gl_Position = proj_matrix * view_matrix * world_coords;
    shadowCoordT = depth_view_proj * world_coords;

    surfaceNormal = normal;
    x_color = material_color;
}
