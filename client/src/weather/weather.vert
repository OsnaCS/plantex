 #version 140
in vec2 position;
out vec2 out_position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    out_position = position;
}
