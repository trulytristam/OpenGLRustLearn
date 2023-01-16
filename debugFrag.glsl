#version 420 core
  
out vec4 vertexColor; // specify a color output to the fragment shader
in vec3 vcolor;

void main()
{
    vertexColor = vec4(vcolor,1.);
}
