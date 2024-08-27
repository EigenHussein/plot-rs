#version 460 core

in vec4 col;
in vec2 txc;
out vec4 Color;

uniform sampler2D TX;

void main(){

    // Color = col;//Vec4(vec3(texture(TX,txc).r),1);
    Color = vec4(vec3(texture(TX,txc).r),1);
}