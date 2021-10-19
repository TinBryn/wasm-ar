attribute vec3 aPosition;
attribute vec3 aNormal;

uniform mat4 uTransform;
uniform mat4 uProjection

varying vec3 vColor;
varying vec3 vNormal;
varying vec3 vPosition;

void main(){
    vColor=(aNormal+1.)*.5;
    vNormal=aNormal*uTransform;
    
    gl_Position=vec4(position,1.)*uTransform*uProjection;
    
}
