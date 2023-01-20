#version 420 core
  
out vec4 vertexColor; // specify a color output to the fragment shader
in vec4 gl_FragCoord;
uniform uint windowSizeX;
uniform float object_count;
uniform uint windowSizeY;
uniform float iTime;
uniform vec3 cPos;
layout (binding = 0) uniform positions {
    float position[1024];
};

layout (binding = 0) uniform orientations {
    float orientation[4096];
};
layout (binding = 0) uniform dims {
    float dimension[1024];
};

float sphereSDF(vec3 p, float r){
    return length(p) - r;
}

float cubeSDF(vec3 p, vec3 b){
    vec3 q = abs(p) - b;
    return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0) - 0.04;
}
vec3 normalize(vec3 a){
    return a/ length(a);
}
float clamp(float x, float a,float b){
    return min(b,max(a,x));
}
float clampn(float x){
    return min(1.,max(0.,x));
}

mat3 getMat(int i){
    i*=9;
    return mat3(orientation[i],orientation[i+1], orientation[i+2],orientation[i+3],orientation[i+4],orientation[i+5], orientation[i+6], orientation[i+7], orientation[i+8]);
}
vec3 getVec(int i){
    i*=3;
    return vec3(position[i],position[i+1],position[i+2]);
}
float map(vec3 p){
    float clos = 100.;
    
    for(float i =0; i< object_count; i+=1.){
        int id = int(i);
        vec3 sP = getVec(id); 
        sP = p - sP;
        mat3 oInvOri = getMat(id); 
        sP = oInvOri*sP;
        clos = min(clos, cubeSDF(sP,vec3(dimension[id*3]/2.,dimension[id*3+1]/2.,dimension[id*3+2]/2.)) );
    }
    return clos;
}
float rayCast(vec3 ro, vec3 rd){
    float t = 0.01;

    for(int i =0; i< 150; i++){
        vec3 p = ro + rd*t;

        float m= map(p);
        
        if(abs(m) < 0.001){
            return t;
        }
        if(t > 200.)break;
        t+= m;

    }

    return -1.;
}
vec3 calcNorm(vec3 p){
    float delt = 0.0001;
    
    vec3 gr = vec3(map(p+vec3(delt,0.,0.))-map(p-vec3(delt,0.,0.)),
                   map(p+vec3(0.,delt,0.))-map(p-vec3(0.,delt,0.)),
                   map(p+vec3(0.,0.,delt))-map(p-vec3(0.,0.,delt))
    );
    return normalize(gr);
}
void main()
{
    vec2 dim = vec2(float(windowSizeX),float(windowSizeY));
    vertexColor = vec4(1.0); 
    vec2 fragCoord = gl_FragCoord.xy/dim; 
    vec2 uv = fragCoord *2.0 - 1.; 
    uv.y *=dim.y/dim.x; 
    vec3 ro = cPos; 
    vec3 rd = vec3(uv,0.7);
    vec3 sunpos = vec3(5.,5.,-4.);
    float di = rayCast(ro,rd);
    vec3 inter = ro + rd* di;
    vec3 normal = calcNorm(inter);

    vec3 sunDot = vec3(clampn(dot(-normal,normalize(inter-sunpos))));
    vec3 sunColor = vec3(0.4,0.2,0.1);
    vec3 skyDot = vec3(clampn(dot(-normal,vec3(0.,-1.,0.)))); 
    vec3 skyColor = vec3(0.2,0.2,0.5);

    //lighting
    vec3 color = vec3(0.);
    sunDot = smoothstep(0.,1. ,sunDot );
    color += sunDot * sunColor* 2.;
    color += pow(sunDot,vec3(140.)) * sunColor* 10.;
    color += (1.-sunDot)* .001;
    color += skyDot * 1.6* skyColor;
    color += (1.-skyDot)* 0.003* skyColor;
    color *= vec3(0.4,0.4,1.2);

    color = pow(color,vec3(0.4545));

    if(di > 0.0){
        vertexColor = vec4(color,1.); 
    }
    else{
        vertexColor = vec4(0.1*(sunColor+skyColor),1.0);
    }
}
