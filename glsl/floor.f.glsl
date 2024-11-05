precision mediump float;

varying vec2 v_texture;

float ease(float t) {
    float p = 2.0 * t * t;
    return t < 0.5 ? p : -p + (4.0 * t) - 1.0;
}

void main() {
    vec2 xy = gl_FragCoord.xy;

    vec3 color_noise = vec3(
    fract(sin(dot(xy, vec2(12.9898, 78.233))) * 43758.5453),
    fract(sin(dot(xy, vec2(93.9898, 67.345))) * 43758.5453),
    fract(sin(dot(xy, vec2(43.332, 93.532))) * 43758.5453)
    ) * (2.0 / 255.0);

    vec4 color = vec4(pow(vec3(75, 172, 232) * 0.8 / 255.0, vec3(2.2)) + color_noise, 1.0);

    float opacity_noise = fract(sin(dot(xy, vec2(12.9898, 67.345))) * 43758.5453) * (2.0 / 255.0);
    float opacity = ease(max(1.0 - length(v_texture) + opacity_noise, 0.0)) * 0.6;
    gl_FragColor = color * opacity;
}