#version 330

uniform sampler2D image;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 out_color;

void main()
{
    out_color = texture(image, i.frag_texcoord);
}
