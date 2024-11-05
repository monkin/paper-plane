precision mediump float;

uniform vec3 u_light_position;
varying vec3 v_position;
varying vec3 v_normal;

void main() {
    float light = abs(dot(normalize(u_light_position - v_position), v_normal));
    vec3 hue = pow(vec3(238, 236, 233) * (1.0 / 255.0), vec3(2.2));

    vec3 color = pow((0.9 + 0.3 * light) * hue, vec3(1.0 / 2.2));

    gl_FragColor = vec4(color, 1);
}