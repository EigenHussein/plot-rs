#version 460 core

layout(location = 0) in vec4 pos;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 txcoord;

uniform mat4 Proj;
uniform mat4 View;

out vec4 col;
out vec2 txc;

void main(){
      gl_Position = Proj*View*vec4(pos.xyz,1);
      col=color;
      txc = txcoord;
}
