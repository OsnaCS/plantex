#version 140

in vec3 x_color;
in vec3 toLight;
in vec3 surfaceNormal;

out vec3 color;

void main() {
    float diffuse = max(0.0, dot(toLight, surfaceNormal));
    color = x_color * 0.1 + x_color * diffuse;
}
