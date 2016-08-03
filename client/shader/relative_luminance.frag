#version 330 core

// ===================================================================
//                   Greyscale with relative luminance
// ===================================================================
//
// Creates a greyscale texture with relative luminance.

out float FragColor;

in VertexData {
    vec2 frag_texcoord;
} i;

uniform sampler2D image;

void main()
{
    float t = 3;
    float tt = 1/t;
    FragColor = log((tt + dot(texture(image, i.frag_texcoord).rgb,
        vec3(0.2126, 0.7152, 0.0722))) * t);
}
