#version 330

uniform sampler2D decal_texture;

in VertexData {
    vec2 frag_texcoord;
} i;

layout(location = 0) out vec4 out_color;

void main() {
    vec3 col = texture(decal_texture, i.frag_texcoord).rgb;


    // transform to greyscale for proper brightness
    if (dot(col, vec3(0.2126, 0.7152, 0.0722)) > 0.5) {
        out_color = vec4(col, 1.0);
    } else {
        out_color = vec4(0);
    }
}
