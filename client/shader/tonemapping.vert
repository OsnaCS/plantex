#version 330

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 texcoord;

out VertexData {
    vec2 frag_texcoord;
} o;

void main() {
    o.frag_texcoord = texcoord;
    gl_Position = position;
}
