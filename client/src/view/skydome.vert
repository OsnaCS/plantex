#version 140

in vec3 position;
in vec3 color;

out vec3 x_color;


uniform mat4 proj_matrix;
uniform mat4 view_matrix;

void main() {

    // glEnable(GL_PRIMITIVE_RESTART);
    // int RESTART_INDEX =-1;
    // glPrimitiveRestartIndex(RESTART_INDEX);




    gl_Position = proj_matrix * view_matrix * vec4(position, 1);

    x_color = color;
}
