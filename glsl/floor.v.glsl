precision highp float;

uniform mat4 u_camera;

attribute vec3 a_position;
attribute vec2 a_texture;

varying vec2 v_texture;

void main() {
    v_texture = a_texture;
    gl_Position = u_camera * vec4(a_position, 1.0);
}
