precision mediump float;

uniform mat4 u_camera;

attribute vec3 a_position;

attribute float i_side;

void main() {
    gl_Position = u_camera * vec4(a_position * vec3(i_side, 1, 1), 1);
}