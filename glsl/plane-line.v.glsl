precision highp float;

uniform mat4 u_camera;

attribute vec3 a_position;

attribute float i_side;

void main() {
    vec3 position = a_position;
    position.x *= i_side;
    gl_Position = u_camera * vec4(position, 1.0);
}
