#version 150

in vec3 x_unit_coords;
out vec3 color;

void main() {
    float temp = (x_unit_coords.x * x_unit_coords.x + x_unit_coords.y * x_unit_coords.y) ;
    if (temp <= 1.0) {
        color = vec3(1.0, 1.0, 1.0 - temp);
    } else {
        discard;
    }
}
