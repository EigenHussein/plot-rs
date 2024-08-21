#version 460 core

in vec4 col;
out vec4 Color;

void main(){

    Color = col;//Vec4(vec3(texture(TX,txc).r),1);

}