#version 330
in vec3 position;
in vec3 point;

uniform vec4 color;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform mat4 scaling_matrix;
uniform int form;
uniform vec3 sun_color;
uniform vec3 sky_light;


out vec2 out_position;
out vec4 out_color;

void main() {

    mat4 world_matrix = mat4 (
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        position.x, position.y, position.z, 1.0);

    mat4 view = view_matrix * world_matrix;

    if (form == 1) {
        view[0][0] = 1.0;
        view[0][1] = 0.0;
        view[0][2] = 0.0;

        view[1][0] = 0.0;
        view[1][1] = 1.0;
        view[1][2] = 0.0;
    }

    if (form == 2 || form == 3) {
        view[0][0] = 1.0;
        view[0][1] = 0.0;
        view[0][2] = 0.0;

        view[1][0] = 0.0;
        view[1][1] = 0.0;
        view[1][2] = 1.0;

        view[2][0] = 0.0;
        view[2][1] = 1.0;
        view[2][2] = 0.0;
    }


    gl_Position =  proj_matrix  * view * scaling_matrix * vec4(point.x, 0.0, point.y, 1.0);
    out_position = vec2(point.x, point.y);
    out_color = color;
}
