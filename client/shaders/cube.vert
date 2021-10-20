# version 300 es

layout(location = 0) in vec3 aPosition;
layout(location = 1) in vec3 aNormal;

uniform mat4 uTransform;
uniform mat4 uProjection;

out vec3 vColor;
out vec3 vNormal;
out vec3 vPosition;

void main() {
    mat3 normalTransform = transpose(inverse(mat3(uTransform)));

    vColor = (aNormal + 1.0) * 0.5;
    vNormal = normalTransform * aNormal;
    vPosition = (uTransform * vec4(aPosition, 1.0)).xyz;

    gl_Position = uProjection * uTransform * vec4(aPosition, 1.0);

}
