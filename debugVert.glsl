#version 330 core
layout (location = 0) in vec2 position; // the position variable has attribute position 0
layout (location = 1) in vec3 color; // the position variable has attribute position 0
  
out vec3 vcolor; // specify a color output to the fragment shader
out int vindex;
vec3 normalize(vec3 v){

   return v/ length(v);
}
vec3 planeinter(vec3 p, float zplane){
    vec3 rd = normalize(p);
    float r = 1./dot(rd,vec3(0.,0.,1.)); 
    return rd * zplane * r;
}
void main()
{

    // gl_Position = vec4(planeinter(position, 0.7).xy, 0.0, 1.0); // see how we directly give a vec3 to vec4's constructor
    gl_Position = vec4(position.xy,0.0, 1.0); // see how we directly give a vec3 to vec4's constructor
    vcolor = color;


}