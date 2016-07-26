#version 330

uniform sampler2D decal_texture;
// uniform Uniforms {
// } u;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 frag_output;

void main() {
    frag_output = vec4(texture(decal_texture, i.frag_texcoord).rgb, 1.0);
}
