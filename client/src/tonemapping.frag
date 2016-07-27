#version 330

uniform sampler2D decal_texture;
uniform float exposure;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 frag_output;

void main() {
    const float gamma = 1.0;

    // get color and use simple tone mapping
    vec3 color = texture(decal_texture, i.frag_texcoord).rgb;
    vec3 mapped = color / (color + vec3(1.0));

    //exposure used for day night cycle
    mapped = vec3(1.0) - exp(-color * exposure);
    //gamma correction
    mapped = pow(mapped, vec3(1.0 / gamma));
    frag_output = vec4(mapped, 1.0);
}
