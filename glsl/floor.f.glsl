precision mediump float;

varying vec2 v_texture;

void main() {
    float opacity = pow(max(1.0 - length(v_texture), 0.0), 1.0 / 2.2);
    gl_FragColor = vec4(1.0); // vec4(0.0, 0.0, 0.0, opacity);
}