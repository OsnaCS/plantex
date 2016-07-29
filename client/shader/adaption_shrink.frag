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
    FragColor = vec4(texture(image, i.frag_texcoord + vec2(0.5, 0.5)).rgb, 1.0);
}
