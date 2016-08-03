#version 150

in vec3 material_color;
in vec3 tes_normal;
in vec3 tes_color;
in vec3 pos;

out vec3 color;

uniform vec3 sun_dir;

const vec3 sun = normalize(vec3(1.0, 0.0, 1.0));

void main() {
    float diffuse = max(0.0, dot(sun, tes_normal));
    color = tes_color * 0.1 + tes_color * diffuse;

    // apply fog to final color
    float distance = (length(pos) / 130) * (length(pos) / 130);
    if (distance > 1) {
        distance = 1;
    }
    float fog_time = -(sun_dir.z / 3);

    if (fog_time < 0) {
        fog_time = 0;
    }

    vec3 fog_color = vec3(0.05 + fog_time, 0.05 + fog_time, 0.1 + fog_time);
    color = mix(color,fog_color,distance);
}
