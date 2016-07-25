#version 140

uniform mat4 matrix;

in vec4 position;
in vec2 texcoord;

smooth out vec2 frag_texcoord;

void main() {
    frag_texcoord = texcoord;
    gl_Position = matrix * position;
}
