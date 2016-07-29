#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec3 texCoords;

out ShadowData{
    vec3 FragPos;
    vec3 Normal;
    vec3 TexCoords;
    vec4 FragPosLightSpace;
} o;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform mat4 lightSpaceMatrix;

void main()
{
    // gl position
    gl_Position = projection * view * model * vec4(position, 1.0);
    // frag position
    o.FragPos = vec3(model * vec4(position, 1.0));
    // normals
    o.Normal = transpose(inverse(mat3(model)));
    // texture coords
    o.TexCoords = texCoords;
    // Frag position in light space
    o.FragPosLightSpace = lightSpaceMatrix * vec4(o.FragPos, 1.0);
}
