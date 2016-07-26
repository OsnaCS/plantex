#version 140

in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 x_color;
out vec3 surfaceNormal;
out vec3 toLight;

uniform vec3 offset;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;

const vec3 sun = vec3(1.0, 0.0, 1.0);

void main() {
    gl_Position = proj_matrix * view_matrix * vec4(position + offset, 1);

    surfaceNormal = normal;
    toLight = sun;
    x_color = color;
}
