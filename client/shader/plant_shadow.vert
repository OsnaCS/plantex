#version 400

in vec3 position;
in vec3 color;
in vec3 normal;
in vec3 offset;

out vec3 vPosition;
out vec3 material_color;
out vec3 surfaceNormal;
out vec3 vOffset;


uniform mat4 proj_matrix;
uniform mat4 view_matrix;

void main() {
    gl_Position = proj_matrix * view_matrix * vec4(position + offset, 1);
}
