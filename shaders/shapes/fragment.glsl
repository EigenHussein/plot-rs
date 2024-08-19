#version 460 core

in vec3 col;
out vec4 Color;

void main(){

    Color = vec4(col,1.0);//Vec4(vec3(texture(TX,txc).r),1);

}