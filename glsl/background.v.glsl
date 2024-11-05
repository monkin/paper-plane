precision highp float;

attribute vec2 a_position;

varying mediump float v_y;

void main() {
    v_y = (a_position.y + 1.0) * 0.5;
    gl_Position = vec4(a_position, 0.0, 1.0);
}