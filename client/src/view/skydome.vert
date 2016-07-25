#version 140

in vec3 position;
in vec3 color;
in float theta;
in float phi;

out vec3 x_color;


uniform mat4 proj_matrix;
uniform mat4 view_matrix;

void main() {

    // glEnable(GL_PRIMITIVE_RESTART);
    // int RESTART_INDEX =-1;
    // glPrimitiveRestartIndex(RESTART_INDEX);




    gl_Position = proj_matrix * view_matrix * vec4(position, 1);

    // x_color = vec3(theta, 0, phi);
    x_color = vec3(phi/6.28);
    // x_color = color;
}
