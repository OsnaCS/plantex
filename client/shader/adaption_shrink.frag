#version 330 core

// ===================================================================
//                   Brightness Adaption Shrinking
// ===================================================================
//
//

out float FragColor;

in VertexData {
    vec2 frag_texcoord;
} i;

uniform sampler2D image;

void main()
{
    FragColor = texture(image, i.frag_texcoord).r;
}
