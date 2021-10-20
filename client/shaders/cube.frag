# version 300 es

precision mediump float;

in vec3 vNormal;
in vec3 vColor;
in vec3 vPosition;

out vec4 FragColor;

const vec3 lightDirection = normalize(vec3(.2, .3, -.5));

void main() {
    vec3 normal = normalize(vNormal);

    float diffuse = dot(lightDirection, normal);

    vec3 V = normalize(-vPosition);
    vec3 H = normalize(V + lightDirection);

    float specular = dot(normal, H);

    specular = pow(clamp(specular, 0.0, 1.0), 400.0);

    diffuse = clamp(diffuse, 0.3, 1.0);

    FragColor = vec4(vColor * diffuse + specular, 1.0);
}
