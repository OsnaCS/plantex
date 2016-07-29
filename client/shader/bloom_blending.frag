#version 330

uniform sampler2D world_tex;
uniform sampler2D bloom_tex;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 out_color;

void main()
{
    vec3 hdr_color = texture(world_tex, i.frag_texcoord).rgb;
    vec3 bloom_color = texture(bloom_tex, i.frag_texcoord).rgb;
    out_color = vec4(hdr_color + bloom_color, 1.0);
}
