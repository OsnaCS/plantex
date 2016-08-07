#version 140

in vec4 shadowCoord;
in vec3 x_material_color;
in vec3 surfaceNormal;
in vec3 x_position;
in float x_radius;
in vec2 x_tex_coords;
flat in int x_ground;
in vec3 pos;

out vec3 color;

// Vector from the camera to the sun
uniform vec3 sun_dir;
uniform sampler2D shadow_map;
uniform vec3 sun_color;
uniform vec3 sky_light;

// Normals to bump mapping the textures
uniform sampler2D normal_sand;
uniform sampler2D normal_snow;
uniform sampler2D normal_grass;
uniform sampler2D normal_stone;
uniform sampler2D normal_dirt;
uniform sampler2D normal_mulch;


// Surface textures
uniform sampler2D sand_texture;
uniform sampler2D grass_texture;
uniform sampler2D snow_texture;
uniform sampler2D stone_texture;
uniform sampler2D dirt_texture;
uniform sampler2D mulch_texture;

const float SHADOW_BIAS = 0.001;    // FIXME does this even work?
const float AMBIENT = 0.2;


float lightCoverage(vec2 moments, float fragDepth) {
    float E_x2 = moments.y;
    float Ex_2 = moments.x * moments.x;
    float variance = E_x2 - Ex_2;
    float mD = moments.x - fragDepth;
    float mD_2 = mD * mD;
    float p = variance / (variance + mD_2);
    return min(max(p, fragDepth <= moments.x ? 1.0 : 0.0), 1.0);
}

/// Calculates Tangent Binormal Normal (tbn) Matrix
mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
    vec3 dp1 = dFdx(pos);
    vec3 dp2 = dFdy(pos);
    vec2 duv1 = dFdx(uv);
    vec2 duv2 = dFdy(uv);

    vec3 dp2perp = cross(dp2, normal);
    vec3 dp1perp = cross(normal, dp1);
    vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
    vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;

    float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
    return mat3(T * invmax, B * invmax, normal);
}

void main() {
    // vec3 lightCoords = shadowCoord.xyz;
    vec3 lightCoords = shadowCoord.xyz / shadowCoord.w;
    lightCoords = lightCoords * 0.5 + 0.5;
    float lit;
    if (lightCoords.x < 0 || lightCoords.x > 1 || lightCoords.y < 0 || lightCoords.y > 1) {
        // Outside of shadow map. Guess brightness from sun angle.
        float sunDot = dot(vec3(0, 0, 1), normalize(sun_dir));
        lit = clamp(-sunDot * 3.0, 0, 1);
    } else {
        vec2 moments = texture(shadow_map, lightCoords.xy).xy;
        lit = lightCoverage(moments, lightCoords.z - SHADOW_BIAS);
        lit = mix(1.0, lit, 0.9);
        // lit = 1;
    }
    if (sun_dir.z > 0) {
        // lit *= max(0, 1 + dot(vec3(0, 0, 1), normalize(sun_dir)) * 2);
        lit = 0;
    }

    // ==================
    // LIGHT CALCULATIONS
    // ==================

    // Calculate normal map relative to surface
    vec3 normal_map;
    // Correcting the height to fit the height to the texture coordinates
    vec2 tex = vec2(x_tex_coords.x, fract(x_tex_coords.y));

    // Determine which surface texture to use
    vec3 diffuse_color;

    if (x_ground == 1) {
        normal_map = texture(normal_grass, tex).rgb;
        diffuse_color = texture(grass_texture, x_tex_coords).rgb;
    } else if (x_ground == 2) {
        normal_map = texture(normal_sand, tex).rgb;
        diffuse_color = texture(sand_texture, x_tex_coords).rgb;
    } else if (x_ground == 3) {
        normal_map = texture(normal_snow, tex).rgb;
        diffuse_color = texture(snow_texture, x_tex_coords).rgb;
    } else if (x_ground == 4) {
        normal_map = texture(normal_dirt, tex).rgb;
        diffuse_color = texture(dirt_texture, x_tex_coords).rgb;
    } else if (x_ground == 5) {
        normal_map = texture(normal_stone, tex).rgb;
        diffuse_color = texture(stone_texture, x_tex_coords).rgb;
    } else if (x_ground == 7) {
        normal_map = texture(normal_mulch, tex).rgb;
        diffuse_color = texture(mulch_texture, x_tex_coords).rgb;
    }

    // Calculate Tangent Binormal Normal (tbn) Matrix to multiply with normal_map
    // to convert to real normals
    mat3 tbn = cotangent_frame(normal_map, x_position, x_tex_coords);
    vec3 real_normal = normalize(tbn * -(normal_map * 1.6 - 1.0));

    // Calculate diffuse light component
    float diffuse = max(0.0, dot(-normalize(sun_dir), real_normal));


    diffuse_color *= x_material_color;

    // DEBUG: for showing normal map as texture
    // vec3 normal_color_map = texture(normal_sand, x_tex_coords).rgb;

    vec3 specular_color = vec3(1.0, 1.0, 1.0);
    vec3 half_direction = sun_dir;
    float specular = pow(max(dot(half_direction, real_normal), 0.0), 16.0);

    // if (x_ground == 1) {
    //     diffuse *= 40;
    //     specular *= 40;
    // } else if (x_ground == 2) {
    //     diffuse *= 35;
    //     specular *= 35;
    // } else if (x_ground == 3) {
    //     diffuse *= 10;
    //     specular *= 10;
    // } else if (x_ground == 4) {
    //     diffuse *= 35;
    //     specular *= 35;
    // } else if (x_ground == 5) {
    //     diffuse *= 20;
    //     specular *= 20;
    // } else if (x_ground == 7) {
    //     diffuse *= 30;
    //     specular *= 30;
    // }

    // Final color calculation
    // color = diffuse_color * AMBIENT + diffuse_color * diffuse * lit + diffuse_color * specular;
    color = diffuse_color * sky_light +  lit * sun_color * diffuse_color * (diffuse + specular);

    // Set Border to distinguish hexagons
    if (x_radius > 0.98) {
        // color *= 0.7;
        color = log(1 + color);
    }

    // apply fog to final color
    float distance = (length(pos) / 130) * (length(pos) / 130);
    if (distance > 1) {
        distance = 1;
    }
    float fog_time= -(sun_dir.z / 3) * 30;


    if (fog_time < 0) {
        fog_time = 0;
    }

    vec3 fog_color = vec3(0.05 + fog_time, 0.05 + fog_time, 0.1 + fog_time);
    vec3 tmp_color = mix(color, fog_color, distance/1.5);
    color = tmp_color;
    // color = tmp_color * ;
    // color += tmp_color * sky_light;
}
