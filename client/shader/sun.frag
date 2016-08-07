#version 150

in vec3 x_unit_coords;
out vec4 color;
uniform vec3 sun_pos;

void main() {
    float temp = (x_unit_coords.x * x_unit_coords.x + x_unit_coords.y * x_unit_coords.y) ;
    if (temp <= 1.0) {
        color = vec4(1.2, 1.1 - (1.0- sun_pos.z)/1.45, (1.1) - (1.0 - sun_pos.z)/1.0, 1)*10.0;
    } else {
        discard;
    }
}
