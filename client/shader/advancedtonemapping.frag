#version 330

uniform sampler2D decal_texture;
uniform float exposure;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 frag_output;

const float shoulder_strength = 0.15;
const float linear_strength = 0.50;
const float linear_angle = 0.10;
const float toe_strength = 0.20;
const float toe_numerator = 0.02;
const float toe_denominator = 0.30;
const float linear_white = 11.2;

// A gamma value of 2.2 is a default gamma value that
// roughly estimates the average gamma of most displays.
// sRGB color space
const float gamma = 2.2;

//other set of values that seem to look good
/*
const float shoulder_strength = 0.22;
const float linear_strength = 0.30;
const float linear_angle = 0.10;
const float toe_strength = 0.20;
const float toe_numerator = 0.01;
const float toe_denominator = 0.30;
const float linear_white = 11.2; // is the smallest luminance that will be mapped to 1.0
*/

// filmic tonemapping ala uncharted 2
// note if you use it you have to redo all your lighting
// you can not just switch it back on if your lighting is for
// no dynamic range.
vec3 tonemap(vec3 x)
{
    return (
        (x*(shoulder_strength*x+linear_angle*linear_strength)+toe_strength*toe_numerator)/
        (x*(shoulder_strength*x+linear_strength)+toe_strength*toe_denominator)
        )
        -toe_numerator/toe_denominator;
}

void main() {
    vec3 color = texture(decal_texture, i.frag_texcoord).rgb;
    //add Exposure
    color *= exposure;
    //exposure BIAS
    color *= 2.0;

    color = tonemap(color);
    vec3 whitecsale = 1.0/tonemap(vec3(linear_white));
    color *= whitecsale;
    //gamma
    color = pow(color, vec3(1/gamma));
    frag_output = vec4(color, 1.0);
}
