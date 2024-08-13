#version 460 core

out vec4 Color;

in vec4 v_col;
in vec2 v_txc;

uniform sampler2D TX;

void main(){

 vec3 tmp = texture(TX,v_txc,0).rgb;

    for(int i = 1; i < 8; i++) {
      for (int j=-3;j<=3;j++){
         for(int k=-3;k<=3;k++){
            tmp += texture(TX, v_txc+0.005*vec2(j,k), i).rgb/(2*49.0*2.0);
         }
      }
    }

 Color = vec4(tmp,1);

}