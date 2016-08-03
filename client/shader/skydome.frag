#version 150

in vec3 x_unit_coords;

uniform vec3 u_sun_pos;
uniform sampler2D u_star_map;


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

    float lum_factor = 10.0; //change this into a constant

    vec3 upperblue_color = vec3 (12.0, 43.0, 80.0)*lum_factor;
    vec3 blue_color = vec3 (22.0, 77.0, 142.0)*lum_factor;
    vec3 lightblue_color = vec3 (62.0, 134.0, 142.0)*lum_factor;
    vec3 dirtyblue_color = vec3 (105.0, 142.0, 137.0)*lum_factor;
    vec3 dirtyyellow_color = vec3 (170.0, 142.0, 85.0)*lum_factor;
    vec3 yellow_color = vec3 (255.0, 125.0, 0.0)*lum_factor;
    vec3 red_color = vec3 (255.0, 0.0, 0.0)*lum_factor;
    vec3 bottomred_color = vec3 (120.0, 0.0, 0.0)*lum_factor;
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
    // goes from 0..2*PI
    float phi = atan(unit_vector.y, unit_vector.x) ;

    // Calculate dummy blue gradient sky color
    vec3 high_noon_color = vec3(((theta / PI)-0.2)*0.5,((theta / PI)-0.1)*0.5,1.0)*lum_factor;
    sunset_color = sunset_color / 255;
    vec3 nightblue_color = (vec3 (0.0, 0.0, 11.0) / 255)*lum_factor ;

    float nighttime = -0.2;
    float sunrise_start = 0.0;
    float sunset_start = 0.0;
    float high_noon_start = 0.3;

    float sun_z = normalize(u_sun_pos).z;
    float sun_x = normalize(u_sun_pos).x;
    float sun_y = normalize(u_sun_pos).y;

    float sun_phi = atan(sun_y, sun_x) ;

    // distance between current vertex and sun in phi direction
    // divided by 2*PI to get a value between 0 and 1
    // (where 1 corresponds to 2*PI (360 degrees))
    float phi_diff = abs(phi-sun_phi)/(2*PI);
    if (phi_diff > 0.5) {
        phi_diff = 1.0 - phi_diff;
    }

    phi_diff = 1 - 2 * phi_diff;
    phi_diff = phi_diff*phi_diff;
    phi_diff = 1 - phi_diff;
    // phi_diff *= 2.2;
    // if (phi_diff>1) {
    //     phi_diff=1;
    // }

    float theta_tmp = 1 - clamp(theta, 0.0, 1.0);


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
    // night to sunrise OR sunset to night
    } else if (sun_z >= nighttime && sun_z < sun_start) {
        float size = sun_start - nighttime;
        float diff = sun_z - nighttime;
        float percent= diff/size;
        color = mix(nightblue_color, sunset_color, percent);
        color = mix(color, nightblue_color, phi_diff);
        color = mix(color, nightblue_color, theta_tmp);
    // sunrise to high_noon OR high_noon to sunset
    } else if (sun_z >= sun_start && sun_z < high_noon_start) {
        float size = high_noon_start - sun_start;
        float diff = sun_z - sun_start;
        float percent= diff/size;
        // color = mix(sunset_color, high_noon_color, percent);
        color = mix(sunset_color, nightblue_color, phi_diff);
        color = mix(color, nightblue_color, theta_tmp);
        color = mix(color, high_noon_color, percent);
    // high_noon
    } else if (sun_z >= high_noon_start) {
        color = high_noon_color;
    }

    // add stars

    // later better, so the top of the sky has stars too
    // float star = texture(u_star_map, vec2(theta*0.8, 0.5 + theta*phi*0.8));
    vec3 test = normalize(vec3(sin(theta)*cos(phi),sin(theta)*sin(phi),0));
    test *= theta;
    float star = texture(u_star_map, vec2(test.x ,test.y));

    vec3 star_color = vec3(0.0, 0.0, 0.0);

    // float star_value = 1 + theta * 0.01;

    star_color = vec3(max(0, (star - 0.48)) * 25)*0.4;
    color = color + star_color;


    // if (0 < ((star - 0.48) * 25)) {
    //     color = star_color;
    // }
}
