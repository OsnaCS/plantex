#version 150

in vec4 shadowCoord;
in vec3 material_color;
in vec3 tes_normal;
in vec3 tes_color;
in vec3 pos;

out vec3 color;

// Vector from the camera to the sun
uniform vec3 sun_dir;
uniform sampler2D shadow_map;
uniform vec3 sun_color;
uniform vec3 sky_light;

const float SHADOW_BIAS = 0.001;    // FIXME does this even work?
const float AMBIENT = 0.1;


float lightCoverage(vec2 moments, float fragDepth) {
    float E_x2 = moments.y;
    float Ex_2 = moments.x * moments.x;
    float variance = E_x2 - Ex_2;
    float mD = moments.x - fragDepth;
    float mD_2 = mD * mD;
    float p = variance / (variance + mD_2);
    return min(max(p, fragDepth <= moments.x ? 1.0 : 0.0), 1.0);
}

void main() {
    vec3 lightCoords = shadowCoord.xyz / shadowCoord.w;
    // vec3 lightCoords = shadowCoord.xyz;
    lightCoords = lightCoords * 0.5 + 0.5;

    float sunDot = dot(vec3(0, 0, 1), normalize(sun_dir));
    // sunDot = 1;
    float lit;
    if (lightCoords.x < 0 || lightCoords.x > 1 || lightCoords.y < 0 || lightCoords.y > 1) {
        // Outside of shadow map. Guess brightness from sun angle.
        lit = clamp(-sunDot * 3.0, 0, 1);
    } else {
        vec2 moments = texture(shadow_map, lightCoords.xy).xy;
        lit = lightCoverage(moments, lightCoords.z - SHADOW_BIAS);
        // if (sun_dir.z > 0) {
        //     lit = 0;
        // }
        // lit = 1;
    }

    float diffuse = max(0.0, dot(-normalize(sun_dir), normalize(tes_normal)));

    vec3 tmp_color = tes_color * sky_light + tes_color * diffuse * lit * sun_color;
    // vec3 tmp_color = diffuse * sun_color;
    // vec3 tmp_color = tes_color * AMBIENT + tes_color * diffuse * lit;

    // apply fog to final color
    float distance = (length(pos) / 130) * (length(pos) / 130);
    if (distance > 1) {
        distance = 1;
    }
    float fog_time = -(sun_dir.z / 3) * 30;

    if (fog_time < 0) {
        fog_time = 0;
    }

    vec3 fog_color = vec3(0.05 + fog_time, 0.05 + fog_time, 0.1 + fog_time);
    tmp_color = mix(tmp_color, fog_color, distance/1.5);
    // color = tmp_color * sky_light;
    // color += tmp_color * sun_color;
    color = tmp_color;
}
