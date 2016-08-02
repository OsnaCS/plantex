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
    FragColor = vec4(texture(image, i.frag_texcoord).rgb, 1.0);
}
