in vec3 position;
in vec3 normal;

uniform vec3 outline_pos;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform mat4 transformation;

void main() {
    gl_Position = proj_matrix * view_matrix  * vec4(position.x+outline_pos.x,position.y+outline_pos.y,position.z+outline_pos.z, 1);
}
