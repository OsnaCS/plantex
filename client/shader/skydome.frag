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
        sunset_color = mix(blue_color, upperblue_color, percent);
    } else if (z < blue && z >= lightblue) {
        float size = blue - lightblue;
        float diff = z - lightblue;
        float percent = diff/size;
        sunset_color = mix(lightblue_color, blue_color, percent);
    } else if (z < lightblue && z >= dirtyblue) {
        float size = lightblue - dirtyblue;
        float diff = z - dirtyblue;
        float percent = diff/size;
        sunset_color = mix(dirtyblue_color, lightblue_color, percent);
    } else if ( z < dirtyblue && z >= dirtyyellow) {
        float size = dirtyblue - dirtyyellow;
        float diff = z - dirtyyellow;
        float percent = diff/size;
        sunset_color = mix(dirtyyellow_color, dirtyblue_color, percent);
    } else if (z < dirtyyellow && z >= yellow) {
        float size = dirtyyellow - yellow;
        float diff = z - yellow;
        float percent = diff/size;
        sunset_color = mix(yellow_color, dirtyyellow_color, percent);
    } else if (z < yellow && z >= red) {
        float size = yellow - red;
        float diff = z - red;
        float percent = diff/size;
        sunset_color = mix(red_color, yellow_color, percent);
    } else if (z < red && z >= bottomred) {
        float size = red - bottomred;
        float diff = z - bottomred;
        float percent = diff/size;
        sunset_color = mix(bottomred_color, red_color, percent);
    } else if (z < bottomred && z >= black) {
        float size = bottomred - black;
        float diff = z - black;
        float percent = diff/size;
        sunset_color = mix(black_color, bottomred_color, percent);
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

// SHIT
    phi = acos( (unit_vector.x) / sqrt(unit_vector.x * unit_vector.x + unit_vector.y * unit_vector.y) );

    if (unit_vector.y < 0) {
        phi = 2*PI - phi;
    }

    // Calculate dummy blue gradient sky color
    vec3 high_noon_color = vec3((theta / PI)-0.2,(theta / PI)-0.1,1.0);
    sunset_color = sunset_color / 255;
    vec3 nightblue_color = vec3 (0.0, 0.0, 41.0) / 255;

    float nighttime = -0.2;
    float sunrise_start = 0.0;
    float sunset_start = -0.15;
    float high_noon_start = 0.3;

    float sun_z = normalize(u_sun_pos).z;
    float sun_x = normalize(u_sun_pos).x;
    float sun_y = normalize(u_sun_pos).y;
    float sun_phi = acos(sun_x);

// SHIT
    sun_phi = acos( (sun_x) / sqrt(sun_x * sun_x + sun_y * sun_y) );

    if (sun_y < 0) {
        sun_phi = 2*PI - sun_phi;
    }

    // distance between current vertex and sun in phi direction
    // divided by 2*PI to get a value between 0 and 1
    // (where 1 corresponds to 2*PI (360 degrees))
    float phi_diff = abs(phi-sun_phi)/(2*PI);
    if (phi_diff > 0.5) {
        phi_diff = 1.0 - phi_diff;
    }

    phi_diff *= 2.0;

    float sun_start;

    if (sun_x > 0) {
        sun_start = sunrise_start;
    } else {
        sun_start = sunset_start;
    }

    // sky colors corresponding to time
    // night
    if (sun_z < nighttime) {
        color = nightblue_color;
        // color = vec3(phi_diff, 0,0 );

    // night to sunrise OR sunset to night
    } else if (sun_z > nighttime && sun_z < sun_start) {
        float size = sun_start - nighttime;
        float diff = sun_z - nighttime;
        float percent= diff/size;
        color = (nightblue_color + (sunset_color - nightblue_color) * percent);
        color = mix(color, nightblue_color, phi_diff);
        // color = vec3(phi_diff, 0,0 );

    // sunrise to high_noon OR high_noon to sunset
    } else if (sun_z > sun_start && sun_z < high_noon_start) {
        float size = high_noon_start - sun_start;
        float diff = sun_z - sun_start;
        float percent= diff/size;
        color = (sunset_color + (high_noon_color - sunset_color) * percent);
        color = mix(color, nightblue_color, phi_diff);
        // color = vec3(phi_diff, 0,0 );

        // color = mix(vec3(1,0,0), vec3(0,1,0), phi_diff);

    // high_noon
    } else if (sun_z > high_noon_start) {
        color = high_noon_color;
        // color = vec3(phi_diff, 0,0 );

    }



}
