#version 460 core


layout(location = 0) in vec4 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 txcoord;

out vec4 v_col;
out vec2 v_txc;

void main(){
    gl_Position = position;
    v_col = color;
    v_txc = txcoord;
}