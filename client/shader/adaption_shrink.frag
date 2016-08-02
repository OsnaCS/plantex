#version 330 core

// ===================================================================
//                   Brightness Adaption Shrinking
// ===================================================================
//
//

out vec2 FragColor;

in VertexData {
    vec2 frag_texcoord;
} i;

uniform sampler2D image;

void main()
{
    FragColor = vec2(texture(image, i.frag_texcoord).r, 1.0);
}
