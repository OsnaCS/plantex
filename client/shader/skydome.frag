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

    float z = unit_vector.z;

    // float upperblue = 1.0;
    // float blue = 0.859;
    // float lightblue = 0.659;
    // float dirtyblue = 0.416;
    // float dirtyyellow = 0.219;
    // float yellow = 0.106;
    // float red = 0.0;
    // float bottomred = -0.1;
    // float black = -1.0;

    float upperblue = 1.0;
    float blue = 0.729;
    float lightblue = 0.459;
    float dirtyblue = 0.366;
    float dirtyyellow = 0.169;
    float yellow = 0.056;
    float red = 0.0;
    float bottomred = -0.1;
    float black = -0.3;

    vec3 upperblue_color = vec3 (12.0, 43.0, 80.0);
    vec3 blue_color = vec3 (22.0, 77.0, 142.0);
    // vec3 lightblue_color = vec3 (75.0, 145.0, 159.0);
    vec3 lightblue_color = vec3 (62.0, 134.0, 142.0);
    vec3 dirtyblue_color = vec3 (105.0, 142.0, 137.0);
    vec3 dirtyyellow_color = vec3 (170.0, 142.0, 85.0);
    vec3 yellow_color = vec3 (255.0, 125.0, 0.0);
    vec3 red_color = vec3 (255.0, 0.0, 0.0);
    vec3 bottomred_color = vec3 (120.0, 0.0, 0.0);
    vec3 black_color = vec3 (0.0, 0.0, 0.0);

    vec3 sunset_color;

    if (z <= upperblue && z >= blue) {
        // size of section of sky between these two colors
        float size = upperblue - blue;
        // difference of position of `z` to the lower border
        float diff = z - blue;
        // percentage of shift to the upper border
        float percent = diff/size;
        // difference between the two colors will be shifted
        // on the lower border the difference to the upper border
        // (times a shift-`percent`) is added
        sunset_color = (blue_color + (upperblue_color - blue_color) * percent);
    } else if (z < blue && z >= lightblue) {
        float size = blue - lightblue;
        float diff = z - lightblue;
        float percent = diff/size;
        sunset_color = (lightblue_color + (blue_color - lightblue_color) * percent);
    } else if (z < lightblue && z >= dirtyblue) {
        float size = lightblue - dirtyblue;
        float diff = z - dirtyblue;
        float percent = diff/size;
        sunset_color = (dirtyblue_color + (lightblue_color - dirtyblue_color) * percent);
    } else if ( z < dirtyblue && z >= dirtyyellow) {
        float size = dirtyblue - dirtyyellow;
        float diff = z - dirtyyellow;
        float percent = diff/size;
        sunset_color = (dirtyyellow_color + (dirtyblue_color - dirtyyellow_color) * percent);
    } else if (z < dirtyyellow && z >= yellow) {
        float size = dirtyyellow - yellow;
        float diff = z - yellow;
        float percent = diff/size;
        sunset_color = (yellow_color + (dirtyyellow_color - yellow_color) * percent);
    } else if (z < yellow && z >= red) {
        float size = yellow - red;
        float diff = z - red;
        float percent = diff/size;
        sunset_color = (red_color + (yellow_color - red_color) * percent);
    } else if (z < red && z >= bottomred) {
        float size = red - bottomred;
        float diff = z - bottomred;
        float percent = diff/size;
        sunset_color = (bottomred_color + (red_color - bottomred_color) * percent);
    } else if (z < bottomred && z >= black) {
        float size = bottomred - black;
        float diff = z - black;
        float percent = diff/size;
        sunset_color = (black_color + (bottomred_color - black_color) * percent);
    } else {
        sunset_color = black_color;
    }

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
    vec3 high_noon_color = vec3((theta / PI)-0.2,(theta / PI)-0.1,1.0);
    color = high_noon_color;
    color = sunset_color / 255;
}
