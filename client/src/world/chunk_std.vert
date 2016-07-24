#version 140

in vec3 position;
in vec3 color;
out vec3 x_color;

uniform vec3 offset;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform float height;
uniform vec3 material_color;

void main() {
    gl_Position = proj_matrix * view_matrix *
        vec4(position.xy + offset.xy, position.z * height + offset.z, 1);

    x_color = material_color * abs(sin(position.x) * sin(position.x));
}
