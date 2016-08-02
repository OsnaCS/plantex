#version 400

in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 vPosition;
out vec3 material_color;
out vec3 surfaceNormal;

void main() {
    //setting out Variables for Tesselation Controll Shader
    material_color = color;
    surfaceNormal= normal;
    vPosition = position;
}
