// When rendering the shadow map, we aren't interested in color information, so
// this can stay empty (depth will be written automatically).
// FIXME All shadow map fragment shaders are the same, so we should stop
// duplicating the files

#version 140

out vec2 out_color;

void main() {
    float depth = gl_FragCoord.z;

    out_color = vec2(depth, depth * depth);
}
