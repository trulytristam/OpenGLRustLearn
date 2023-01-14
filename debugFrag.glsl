
#version 420 core
  
layout (location = 1) in vec3 color; // the position variable has attribute position 0
out vec4 vertexColor; // specify a color output to the fragment shader
in vec2 gl_FragCoord;
uniform uint windowSizeX;
in vec3 vcolor;
in int vindex;
uniform float object_count;
uniform uint windowSizeY;
uniform float iTime;
uniform vec3 cPos;
layout (binding = 0) uniform lineColors {
    float linecolors[1024];
};

void main()
{
    vertexColor = vec4(vcolor,1.);
}
