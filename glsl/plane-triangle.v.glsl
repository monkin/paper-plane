precision highp float;

uniform mat4 u_camera;

attribute vec3 a_position;
attribute vec3 a_normal;


varying vec3 v_position;
varying vec3 v_normal;

void main() {
    v_position = a_position;
    v_normal = a_normal;
    gl_Position = u_camera * vec4(a_position, 1);
}