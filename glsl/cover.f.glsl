precision mediump float;

uniform float u_opacity;

void main() {
    gl_FragColor = vec4(u_opacity);
}