#version 140

in vec3 position;
out vec3 x_color;

// Height in units, not world coordinates, since the "pillar prototype" has a
// height of one unit.
uniform float height;
uniform vec3 offset;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform vec3 material_color;

void main() {
    gl_Position = proj_matrix * view_matrix *
        vec4(position.xy + offset.xy, position.z * height + offset.z, 1);

    // FIXME: Slightly modify the material color to create some color
    // differences. Remove this once real lighting is used.
    x_color = material_color * abs(sin(position.x) * sin(position.x));
}
