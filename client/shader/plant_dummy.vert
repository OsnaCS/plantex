#version 400

in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 vPosition;

out vec3 material_color;
out vec3 surfaceNormal;

uniform vec3 offset;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;

void main() {
    material_color = color;
    surfaceNormal= normal;
    vPosition = position;
}
