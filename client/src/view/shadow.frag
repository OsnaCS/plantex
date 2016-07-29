#version 330

in ShadowData{
    vec3 FragPos;
    vec3 Normal;
    vec3 TexCoords;
    vec4 FragPosLightSpace;
} i;

uniform sampler2D diffuseTexture;
uniform sampler2D shadowMap;

uniform vec3 lightPos;
uniform vec3 viewPos;

float ShadowCalc(vec4 fragPosLightSpace) {
    // returns fragment light-space position in range [-1,1]
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    // transform to shadow map range [0,1]
    projCoords = projCoords * 0.5 +0.5;

    //projCoords further then the light farplane
    if(projCoords.z > 1.0)
        return 0.0;

    float closestDepth = texture(shadowMap, projCoords).r;
    float currentDepth = projCoords.z;

    // calculat shadow bias to prevent "shadow acne" http://learnopengl.com/img/advanced-lighting/shadow_mapping_acne_diagram.png
    // will result in "peter panning" to fix this tell OpenGl to cull front face.
    float bias = max(0.05 * (1.0 - dot(normal, lightDir)), 0.005);

    // if currentDepth > closestDepth fragment in shadow
    return currentDepth - bias > closestDepth ? 1.0 : 0;
}

// uses the Blinn-Phong lighting model.
void main() {
    vec3 color = texture(diffuseTexture, i.TexCoords).rgb;
    vec3 normal = normalize(i.Normal);
    vec3 lightColor = vec3(1.0);

    // ambient
    vec3 ambient = 0.15 * color;
    // get diffuse
    vec3 lightDir = normalize(lightPos - i.FragPos);
    float diff = max(dot(lightDir, normal),0.0);
    vec3 diffuse = diff * lightColor;
    // specular
    vec3 = viewDir = normalize(viewPos - i.FragPos);
    vec3 = halfwayDir = normalize(lightDir - viewDir);
    float spec = pow(max(dot(normal,halfwayDir),0.0),64.0);
    vec3 specular = spec * lightColor;
    // shadow calculatins (is fragment in shadow =?)
    float shadow = ShadowCalc(i.fragPosLightSpace);
    // result of light
    vec3 lighting (ambient + (1.0 - shadow) * (diffuse + specular)) * color;

    frag_output = vec4(lighting, 1.0);

}
