
float rayCast(vec3 ro, vec3 rd){

       float t = 0.01;

        for(int i =0;i < 200;i++){
            vec3 p = ro + rd*t;

            float m = (p);
            if(m < 0.001)break;
            t+= m;
        }

        return t;
    }
    void mainImage( out vec4 fragColor, in vec2 fragCoord )
{

    vec2 uv = fragCoord/iResolution.xy *2.-1.;
    uv.y*= iResolution.y/iResolution.x; 
    vec3 ro = vec3(0.,0.,-1.);
    vec3 rd = vec3(0.,0.,1.);
    float dist = rayCast(ro,rd,iTime);
    vec3 norm = calcNorm();
    vec3 col =  vec3(1.);
    if (length(uv)<0.3)
        col = vec3(1.,0.,0.);
    fragColor = vec4(col,1); 
        float t = 0.01;

        for(int i =0;i < 200;i++){
            vec3 p = ro + rd*t;

            float m = map(p);
            if(m < 0.001)break;
            t+= m;
        }

        return t;
    }
    void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
}