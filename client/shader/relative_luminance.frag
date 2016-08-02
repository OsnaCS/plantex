#version 330 core

// ===================================================================
//                   Greyscale with relative luminance
// ===================================================================
//
// Creates a greyscale texture with relative luminance.

out vec2 FragColor;

in VertexData {
    vec2 frag_texcoord;
} i;

uniform sampler2D image;

void main()
{
    FragColor = vec2(dot(texture(image, i.frag_texcoord).rgb, vec3(0.2126, 0.7152, 0.0722)), 1.0);
}
