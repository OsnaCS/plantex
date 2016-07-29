#version 140
out vec4 color;
in vec2 out_position;
in vec4 out_color;

uniform int form;

void main() {
    color = vec4(out_color);
    if (length(out_position) > 1) {
        discard;
    }
    if(form == 3){
        color.a = 0.7 - (length(out_position)/2.0);
    }
    if(form == 2){
            color.a = 1 - (length(out_position)/1.5);

    }
}
