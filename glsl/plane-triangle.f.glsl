precision mediump float;

uniform vec3 u_light_position;
varying vec3 v_position;
varying vec3 v_normal;

void main() {
    float light = abs(dot(normalize(u_light_position - v_position), v_normal));
    vec3 color = vec3(pow(0.4 + 0.4 * light, 1.0 / 2.2));

    gl_FragColor = vec4(color, 1);
}