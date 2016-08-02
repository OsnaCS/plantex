#version 140

in vec3 x_color;
in vec3 surfaceNormal;

out vec3 color;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));
void main() {
    float diffuse = max(0.0, dot(sun, surfaceNormal));
    color = x_color * 0.1 + x_color * diffuse;
}
