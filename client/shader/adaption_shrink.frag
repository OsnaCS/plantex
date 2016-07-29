#version 330 core

// ===================================================================
//                   Brightness Adaption Shrinking
// ===================================================================
//
//

out vec4 FragColor;

in VertexData {
    vec2 frag_texcoord;
} i;

uniform sampler2D image;

void main()
{
    vec2 tex_offset = 1.0 / textureSize(image, 0); // gets size of single texel

    FragColor = vec4(texture(image, i.frag_texcoord + vec2(0.5 * tex_offset)).rgb, 1.0);
}
