#version 330

uniform sampler2D decal_texture;
uniform float exposure;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 frag_output;

// adds value to each color channel
vec3 add_val(vec3 color, float nmbr) {
    color[0] += nmbr;
    color[1] += nmbr;
    color[2] += nmbr;
    return color;
}

void main() {
    vec3 color = texture(decal_texture, i.frag_texcoord).rgb;
    // filmic tonemapping ala uncharted 2
    // note if you use it you have to redo all your lighting
    // you can not just switch it back on if your lighting is for
    // no dynamic range.
    const float shoulder_strength = 0.22;
    const float linear_strength = 0.30;
    const float linear_angle = 0.10;
    const float toe_strength = 0.20;
    const float toe_numerator = 0.01;
    const float toe_denominator = 0.30;
    const float linear_white = 2.4; // is the smallest luminance that will be mapped to 1.0
    float toe_angle = toe_numerator / toe_denominator;

    // brick of an equation for each color
    vec3 temp = color * shoulder_strength;

    // top part of fraction
    vec3 high = add_val(temp, linear_strength * linear_angle);
    high *= color;
    high = add_val(high, toe_strength * toe_numerator);
    // bottom part of fraction
    vec3 div = add_val(temp, linear_strength);
    div *= color;
    div = add_val(div, toe_strength * toe_denominator);

    // put all together
    color = add_val(high/div, -toe_angle);

    frag_output = vec4(color / linear_white, 1.0);
}
