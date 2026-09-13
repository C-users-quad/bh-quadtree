#version 330

in vec2 f_local;
in float f_pixel_size;
in vec4 f_rgba;

out vec4 p_rgba;

void main() {
    float dist = length(f_local);
    float alpha = 1.0 - smoothstep(1.0 - 10.0 * f_pixel_size, 1.0, dist);
    if (alpha < 0.001) discard;
    p_rgba = vec4(f_rgba.rgb, f_rgba.a * alpha);
}
