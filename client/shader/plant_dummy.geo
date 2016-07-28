#version 400

in vec3 x_color;
in vec3 surfaceNormal;

out vec3 xcolor;
out vec3 normal;

void main() {

    xcolor= x_color;
    normal=surfaceNormal;


/*vec3 A = tePosition[2] - tePosition[0];
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
