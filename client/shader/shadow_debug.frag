#version 330

uniform sampler2D decal_texture;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 frag_output;

void main() {
    float val = texture(decal_texture, i.frag_texcoord).r;
    if (val == 1.0) {
        frag_output = vec4(1, 0, 0, 1);
    } else {
        frag_output = vec4(val);
    }
}
