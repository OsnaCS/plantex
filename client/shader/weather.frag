#version 140
out vec4 color;
in vec2 out_position;
in vec4 out_color;

void main() {
    color = vec4(out_color);
    if (length(out_position) > 1) {
        discard;
    }
}
