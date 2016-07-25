#version 140

uniform sampler2D decal_texture;
uniform sampler2D lighting_texture;

smooth in vec2 frag_texcoord;
out vec4 frag_output;

void main() {
    vec4 lighting_value = texture(lighting_texture, frag_texcoord);
    frag_output = vec4(texture(decal_texture, frag_texcoord).rgb, 1.0);
}
