in vec3 position;
in vec3 normal;

uniform vec3 outline_pos;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform mat4 transformation;

void main() {
    gl_Position = proj_matrix * view_matrix * transformation * vec4(position + outline_pos, 1);
}
