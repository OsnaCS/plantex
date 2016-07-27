#version 150

in vec3 x_unit_coords;
out vec4 color;




void main() {

 // glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
  // glEnable(GL_BLEND);

    float temp = (x_unit_coords.x * x_unit_coords.x + x_unit_coords.y * x_unit_coords.y) ;
    if (temp <= 1.0) {
        color = vec4(1.0, 1.0, 1.0 - temp,1);
    } else {

        discard;
       //color = vec4(0,0,0,0);
    }
}
