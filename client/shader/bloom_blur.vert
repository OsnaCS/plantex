#version 330

layout(location = 0) in vec4 in_position;
layout(location = 1) in vec2 in_texcoord;

out VertexData {
    vec2 frag_texcoord;
} o;

void main() {
    o.frag_texcoord = in_texcoord;
    gl_Position = in_position;
}
