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
//uniform float weight[5] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

uniform float weight = 0.227027;
uniform float d12 = 1.38461536;
uniform float d34 = 3.23076704;
uniform float k12 = 0.3162162;
uniform float k34 = 0.07027;

void main()
{
    vec2 TexCoords = i.frag_texcoord;
    vec2 tex_offset = 1.0 / textureSize(image, 0); // gets size of single texel
    // current fragment's contribution:
    vec3 result = texture(image, TexCoords).rgb * weight;
    if(horizontal)
    {
        //first fragment pair
        result += texture(image, TexCoords + vec2(tex_offset.x * d12, 0.0)).rgb * k12;
        result += texture(image, TexCoords - vec2(tex_offset.x * d12, 0.0)).rgb * k12;

        //second fragment pair
        result += texture(image, TexCoords + vec2(tex_offset.x * d34, 0.0)).rgb * k34;
        result += texture(image, TexCoords - vec2(tex_offset.x * d34, 0.0)).rgb * k34;
    }
    else
    {
        //first fragment pair
        result += texture(image, TexCoords + vec2(0.0, tex_offset.x * d12)).rgb * k12;
        result += texture(image, TexCoords - vec2(0.0, tex_offset.x * d12)).rgb * k12;

        //second fragment pair
        result += texture(image, TexCoords + vec2(0.0, tex_offset.x * d34)).rgb * k34;
        result += texture(image, TexCoords - vec2(0.0, tex_offset.x * d34)).rgb * k34;
    }
    FragColor = vec4(result, 1.0);
}
