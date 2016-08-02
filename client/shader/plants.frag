#version 150

in vec3 material_color;
in vec3 tes_normal;
in vec3 tes_color;

out vec3 color;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));

void main() {
    float diffuse = max(0.0, dot(sun, tes_normal));
    color = tes_color * 0.1 + tes_color * diffuse;
}
