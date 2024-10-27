precision highp float;

uniform mat4 u_camera;

attribute vec3 a_position;
attribute vec3 a_normal;

attribute float i_side;

varying vec3 v_position;
varying vec3 v_normal;

void main() {
    v_position = a_position * vec3(i_side, 1, 1);
    v_normal = a_normal * vec3(i_side, 1, 1);
    gl_Position = u_camera * vec4(a_position * vec3(i_side, 1, 1), 1);
}