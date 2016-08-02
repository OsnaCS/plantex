#version 140

// Per-vertex attributes
in vec3 position;
in vec3 normal;
in float radius;
in vec2 tex_coords;

// Per-instance attributes:
// Height in units, not world coordinates, since the "pillar prototype" has a
// height of one unit.
in float height;
in vec3 offset;
in vec3 material_color;
in int ground;

// -----------------------

out vec3 x_position;
out vec3 surfaceNormal;
out float x_radius;
out vec2 x_tex_coords;
out vec3 x_material_color;
flat out int x_ground;

// Vertex/Pixel coordinates in shadow map
out vec4 shadowCoord;

// -----------------------

uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform mat4 depth_view_proj;

void main() {
    vec4 world_coords = vec4(
        position.xy + offset.xy,
        position.z * height + offset.z,
        1);

    gl_Position = proj_matrix * view_matrix * world_coords;
    shadowCoord = depth_view_proj * world_coords;

    surfaceNormal = normal;
    x_material_color = material_color;
    x_radius = radius;
    x_tex_coords = tex_coords;

    if(tex_coords.y > 1.5){
        x_tex_coords.y = height;
    }

    x_position = position;
    x_ground = ground;
}
