#version 140

in float o_theta;
in vec2 pre_phi;
// in vec3 x_color;

out vec3 color;

#define PI 3.141592653589793

void main() {
    vec2 normalized_pre_phi = normalize(pre_phi);

    float phi = acos(normalized_pre_phi.x);
    if (normalized_pre_phi.y < 0) {
        phi = 2*PI - phi;
    }

    color = vec3(phi/(2*PI));
    // color = vec3(phi.x / 2 + 0.5);
    // color = x_color;
}
