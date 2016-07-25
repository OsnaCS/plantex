#version 140
out vec4 color;
in vec2 out_position;

void main() {
    color = vec4(1.0, 1.0, 1.0, 1.0);
    if(length(out_position) > 1) {
        discard;
    }
}
