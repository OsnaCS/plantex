#version 150

in vec3 position;
in vec3 color;
in vec3 normal;
in vec3 offset;

out vec3 pos;
out vec3 tes_normal;
out vec3 tes_color;

// Vertex/Pixel coordinates in shadow map
out vec4 shadowCoord;

uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform vec3 camera_pos;
uniform mat4 depth_view_proj;

void main() {
    tes_color = color;
    tes_normal = normal;

    // projection on camera
    vec3 worldPos = position + offset;
    gl_Position = proj_matrix * view_matrix * vec4(worldPos, 1);

    // position for fog
    pos = worldPos - camera_pos;
    pos = vec3(pos.x, pos.y, 0);

    // yes, this somehow is *also* the world position
    vec4 world = view_matrix * vec4(worldPos, 1);
    gl_Position = proj_matrix * world;
    // coordinates on the shadow map
    shadowCoord = depth_view_proj * vec4(worldPos, 1);
}
