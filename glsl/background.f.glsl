precision mediump float;

varying mediump float v_y;

float ease(float t) {
    float p = 2.0 * t * t;
    return t < 0.5 ? p : -p + (4.0 * t) - 1.0;
}

void main() {
    vec2 xy = gl_FragCoord.xy;

    vec3 noise = vec3(
    fract(sin(dot(xy, vec2(12.9898, 78.233))) * 43758.5453),
    fract(sin(dot(xy, vec2(93.9898, 67.345))) * 43758.5453),
    fract(sin(dot(xy, vec2(43.332, 93.532))) * 43758.5453)
    ) * (1.0 / 255.0);

    vec3 color1 = pow(vec3(235, 237, 255) * 0.85 / 255.0, vec3(2.2));
    vec3 color2 = pow(vec3(235, 237, 255) / 255.0, vec3(2.2));
    vec3 color = pow(mix(color1, color2, ease(v_y)), vec3(1.0 / 2.2)) + noise;
    gl_FragColor = vec4(color, 1);
}