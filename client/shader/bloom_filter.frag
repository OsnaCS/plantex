#version 330

// ===================================================================
//                          Bloom Filter Map
// ===================================================================
//
// Filter bright areas and only map those to the texture.

uniform sampler2D decal_texture;
uniform float bloom_threshhold;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 out_color;

void main() {
    vec3 col = texture(decal_texture, i.frag_texcoord).rgb;


    // transform proper brightness. Values adapt for eye vision, for explanation see:
    // https://en.wikipedia.org/wiki/Luma_%28video%29#Use_of_relative_luminance
    if (dot(col, vec3(0.2126, 0.7152, 0.0722)) > bloom_threshhold) {
        out_color = vec4(col, 1.0);
    } else {
        out_color = vec4(0);
    }
}
