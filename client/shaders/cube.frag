varying vec3 vNormal;
varying vec3 vColor;
varying vec3 vPosition;

const vec3 lightDirection=normalize(vec3(.2,.3,-.5));

void main(){
    vec3 normal=normalize(vNormal);
    
    float diffuse=dot(lightDirection);
    
    vec3 V=normalize(-vPosition);
    vec3 H=normalize(V+lightDirection);
    
    float specular=pow(clamp(dot(normal,H),0.,1.),100);
    
    diffuse=clamp(diffuse,0.,1.);

    gl_FragColor = vec4(color * diffuse + specular, 1.0);
}
