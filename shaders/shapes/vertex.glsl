#version 460 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm;
layout(location = 2) in vec2 txcoord;

uniform mat4 Proj;
uniform mat4 View;

out vec2 txc;

void main(){
      gl_Position = Proj*View*vec4(pos.xyz,1);
       // txc = txcoord;
}
