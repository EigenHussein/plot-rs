#version 460 core

out vec4 Color;

in vec3 txc;
uniform sampler2D TX;

void main(){

   //     Color = vec4(0.1,0.4,0.6,1.0);//Vec4(vec3(texture(TX,txc).r),1);

}