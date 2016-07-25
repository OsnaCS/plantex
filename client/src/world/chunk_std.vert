#version 140

in vec3 position;
in vec3 normal;

out vec3 x_color;
out vec3 toLight;
out vec3 surfaceNormal;

// Height in units, not world coordinates, since the "pillar prototype" has a
// height of one unit.
uniform float height;
uniform vec3 offset;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform vec3 material_color;

const vec3 sun = vec3(1.0, 0.0, 1.0);

void main() {
    gl_Position = proj_matrix * view_matrix * vec4(position.xy + offset.xy, position.z * height + offset.z, 1);
    surfaceNormal = normal;
    toLight = sun;
   
    // FIXME: Slightly modify the material color to create some color
    // differences. Remove this once real lighting is used.
    x_color = material_color;
}
