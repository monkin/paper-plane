precision mediump float;

varying float v_opacity;

void main() {
    vec3 color = vec3(0.6, 0.6, 0.6);
    gl_FragColor = vec4(color * v_opacity, v_opacity);
}