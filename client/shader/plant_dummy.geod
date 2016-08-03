#version 400


uniform mat4 Modelview;
uniform mat3 NormalMatrix;

layout(triangles,invocations= 3) in;
layout(triangle_strip, max_vertices = 3) out;

in vec3 tePosition[3];
in vec3 tePatchDistance[3];

in vec3 tes_color[];
in vec3 tes_normal[];

out vec3 xcolor;
out vec3 normal;


out vec3 gFacetNormal;
out vec3 gPatchDistance;
out vec3 gTriDistance;

void main() {
/*
    xcolor= tes_color[0];
    normal=tes_normal[0];


vec3 A = tePosition[2] - tePosition[0];
vec3 B = tePosition[1] - tePosition[0];

gFacetNormal = NormalMatrix * normalize(cross(A, B));

gPatchDistance = tePatchDistance[0];
gTriDistance = vec3(1, 0, 0);
gl_Position = gl_in[0].gl_Position; EmitVertex();

gPatchDistance = tePatchDistance[1];
gTriDistance = vec3(0, 1, 0);
gl_Position = gl_in[1].gl_Position; EmitVertex();

gPatchDistance = tePatchDistance[2];
gTriDistance = vec3(0, 0, 1);
gl_Position = gl_in[2].gl_Position; EmitVertex();

EndPrimitive();*/

}
