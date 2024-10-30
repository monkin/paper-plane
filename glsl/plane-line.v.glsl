precision highp float;

uniform mat4 u_camera;

attribute vec3 a_position;
attribute float a_opacity;

varying float v_opacity;

void main() {
    v_opacity = a_opacity;
    gl_Position = u_camera * vec4(a_position, 1.0);
}
