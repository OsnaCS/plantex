#version 150

in vec3 x_unit_coords;

uniform vec3 u_sun_pos;

out vec3 color;

#define PI 3.141592653589793

void main() {
    // Calculate spherical coordinates
    vec3 unit_vector = normalize(x_unit_coords);

    // Calculates theta
    // unit_vector.z varies from [-1..1] therefore arccos from unit_vector.z,
    // which is theta, varies from [PI..0] respectively
    float theta = acos(unit_vector.z);

    // Calculates phi
    // unit vector.x varies from [-1..1] therefore arccos from unit_vector.x,
    // varies from [PI..0] respectively
    // But because this is only functional for the upper hemisphere,
    // the phi for the lower hemisphere is calculated in the if statement
    float phi = acos(unit_vector.x);
    if (unit_vector.y < 0) {
        phi = 2*PI - phi;
    }

    // Calculate dummy blue gradient sky color
    color = vec3((theta / PI)-0.2,(theta / PI)-0.1,1.0);
}
