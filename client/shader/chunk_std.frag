#version 140

in vec4 shadowCoord;
in vec3 x_material_color;
in vec3 surfaceNormal;
in vec3 x_position;
in float x_radius;
in vec2 x_tex_coords;
flat in int x_ground;

out vec3 color;

// Vector from the camera to the sun
uniform vec3 sun_dir;
uniform sampler2D shadow_map;

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
uniform ivec2 offset_ax;

// Percentage-closer filtering (square) radius in pixels
const int SHADOW_PCF_RADIUS = 1;

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
    // color = vec3(1, 1, 0);
    // return;
    vec3 lightCoords = shadowCoord.xyz / shadowCoord.w;
    lightCoords = lightCoords * 0.5 + 0.5;
    vec2 moments = texture(shadow_map, lightCoords.xy).xy;
    float lit = max(lightCoverage(moments, lightCoords.z - SHADOW_BIAS), 0.2);

    // ==================
    // LIGHT CALCULATIONS
    // ==================

    // Calculate normal map relative to surface
    // vec3 normal_map = texture(normals, x_tex_coords).rgb;

    vec3 normal_map;
    vec2 tex = vec2(x_tex_coords.x, fract(x_tex_coords.y));

    if (x_ground == 1) {
        normal_map = texture(normal_grass, tex).rgb;
    } else if (x_ground == 2) {
        normal_map = texture(normal_sand, tex).rgb;
    } else if (x_ground == 3) {
        normal_map = texture(normal_snow, tex).rgb;
    } else if (x_ground == 4) {
        normal_map = texture(normal_dirt, tex).rgb;
    } else if (x_ground == 5) {
        normal_map = texture(normal_stone, tex).rgb;
    } else if (x_ground == 7) {
        normal_map = texture(normal_mulch, tex).rgb;
    }



    // Calculate Tangent Binormal Normal (tbn) Matrix to multiply with normal_map
    // to convert to real normals
    mat3 tbn = cotangent_frame(normal_map, x_position, x_tex_coords);
    vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));

    // Calculate diffuse light
    float diffuse = max(0.0, dot(-sun_dir, real_normal));

    // =============
    // Determine which surface texture to use
    // Andy & Helena
    // FIXME Be smarter about this calculation - We simply make the whole color
    // darker
    // TODO: More grounds and make it better ;D

    vec3 diffuse_color;
    if (x_ground == 1) {
        diffuse_color = texture(grass_texture, x_tex_coords).rgb;
    } else if (x_ground == 2) {
        diffuse_color = texture(sand_texture, x_tex_coords).rgb;
    } else if (x_ground == 3) {
        diffuse_color = texture(snow_texture, x_tex_coords).rgb;
    } else if (x_ground == 4) {
        diffuse_color = texture(dirt_texture, x_tex_coords).rgb;
    } else if (x_ground == 5) {
        diffuse_color = texture(stone_texture, x_tex_coords).rgb;
    } else if (x_ground == 7) {
        diffuse_color = texture(mulch_texture, x_tex_coords).rgb;
    }

    diffuse_color *= x_material_color;
    // diffuse_color = x_material_color/100;

    // vec3 diffuse_color = x_material_color;

    // =============

    // TODO: temp fix
    // This is how we should calculate the diffuse color component
    // Substitute in the chosen texture
    // vec3 diffuse_color = texture(my_texture, x_tex_coords).rgb * x_material_color;


    // DEBUG: for showing normal map as texture
    // vec3 normal_color_map = texture(normal_sand, x_tex_coords).rgb;

    // FIXME: specular color calculation is off
    // const vec3 specular_color = vec3(1.0, 1.0, 1.0);
    // vec3 camera_di half_direction = normalize(normalize(-sun_dir) + camera_dir);
    // float specular = pow(max(dot(half_direction, real_normal), 0.0), 16.0);
    // r = normalize(-x_position);
    // vec3

    // Final color calculation
    color = diffuse_color * AMBIENT + diffuse_color * diffuse * lit;
    // color.b = offset_ax.y / (10*16.0) + 0.5;
    // color.rg = vec2(0);
    // color = diffuse_color * AMBIENT + normal_color_map * diffuse;

    // Set Border to distinguish hexagons
    if (x_radius > 0.98) {
        color = vec3(0);
    }
}
