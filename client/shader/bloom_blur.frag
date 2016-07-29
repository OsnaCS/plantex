#version 330 core

// ===================================================================
//                      Simple Gaussian Blur
// ===================================================================
//
// Blurs the filtered Light Map. Use Gaussian Blur affecting the current
// and the 4 in one direction neighboring fragments (along a horizontal or
// vertical line).

out vec4 FragColor;

in VertexData {
    vec2 frag_texcoord;
} i;

uniform sampler2D image;

uniform bool horizontal;  // indicates whether to blur horizontal or vertical

// weights of the gauss curve, weight[0] corresponds to center fragment
uniform float weight[5] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

void main()
{
    vec2 TexCoords = i.frag_texcoord;
    vec2 tex_offset = 1.0 / textureSize(image, 0); // gets size of single texel
    // current fragment's contribution:
    vec3 result = texture(image, TexCoords).rgb * weight[0];
    if(horizontal)
    {
        for(int i = 1; i < 5; ++i)
        {
            result += texture(image, TexCoords + vec2(tex_offset.x * i, 0.0)).rgb * weight[i];
            result += texture(image, TexCoords - vec2(tex_offset.x * i, 0.0)).rgb * weight[i];
        }
    }
    else
    {
        for(int i = 1; i < 5; ++i)
        {
            result += texture(image, TexCoords + vec2(0.0, tex_offset.y * i)).rgb * weight[i];
            result += texture(image, TexCoords - vec2(0.0, tex_offset.y * i)).rgb * weight[i];
        }
    }
    FragColor = vec4(result, 1.0);
}
