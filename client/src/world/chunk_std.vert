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
out vec3 toLight;
out vec3 surfaceNormal;

uniform mat4 proj_matrix;
uniform mat4 view_matrix;

const vec3 sun = vec3(1.0, 0.0, 1.0);

void main() {
    gl_Position = proj_matrix * view_matrix * vec4( position.xy +
                                                    offset.xy, position.z *
                                                    height +
                                                    offset.z, 1);
    surfaceNormal = normal;
    toLight = sun;
    x_color = material_color;
}
