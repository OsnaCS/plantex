#version 330

uniform sampler2D decal_texture;
uniform float exposure;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 frag_output;

void main() {
    //A gamma value of 2.2 is a default gamma value that
    //roughly estimates the average gamma of most displays.
    //sRGB color space
    const float gamma = 2.2;
    vec3 hdrColor = texture(decal_texture, i.frag_texcoord).rgb;
    // Exposure tone mapping
    vec3 mapped = vec3(1.0) - exp(-hdrColor * exposure);
    // Gamma correction
    mapped = pow(mapped, vec3(1.0 / gamma));

    frag_output = vec4(mapped, 1.0);
}
