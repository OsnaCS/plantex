#version 140

in float pre_theta;
in vec2 pre_phi;
in vec3 unit_coords;

out vec3 color;

#define PI 3.141592653589793

void main() {
    // Calculate spherical coordinates
    vec3 unit_vector = normalize(unit_coords);
    float theta = acos(unit_vector.z);

    float phi = acos(unit_vector.x);
    if (unit_vector.y < 0) {
        phi = 2*PI - phi;
    }


    // Calculate color


    color = vec3((theta / PI)-0.2,(theta / PI)-0.1,1.0);

    // color = vec3(0.0,0.0,theta / PI);
}
