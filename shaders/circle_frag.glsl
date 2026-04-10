#version 330

in vec2 f_center;
in vec2 f_pos;
in float f_radius;
in vec4 f_rgba;

out vec4 p_rgba;

void main() {
    vec2 diff = f_pos - f_center;
    if (dot(diff, diff) > f_radius * f_radius) {
        discard;
    }
    p_rgba = f_rgba;
}
