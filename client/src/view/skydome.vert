#version 140

in vec3 position;
in vec2 xy;
in vec3 color;
in float theta;

out float o_theta;
out vec2 pre_phi;
// out vec3 x_color;


#define PI 3.141592653589793


uniform mat4 proj_matrix;
uniform mat4 view_matrix;

// #define PI = 3.141592653589793

void main() {

    // glEnable(GL_PRIMITIVE_RESTART);
    // int RESTART_INDEX =-1;
    // glPrimitiveRestartIndex(RESTART_INDEX);




    gl_Position = proj_matrix * view_matrix * vec4(position, 1);

    // x_color = vec3(0, 0, phi);
    // x_color = vec3(theta, 0, 0);

    // x_color = vec3(theta/PI, 0, phi/2*PI);





    o_theta = theta;
    pre_phi = xy;

    // x_color = vec3(phi/(2*PI), ;



    // x_color = vec3(theta/20, 0, phi/20);

    // x_color = vec3(phi/6.28);
    // x_color = color;
}
