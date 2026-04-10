#version 330

in vec2 pos;
in vec4 rgba;
in float radius;
in vec2 center;

uniform vec2 cam_pos;
uniform float cam_zoom;
uniform float inverse_aspect;

out vec2 f_center;
out vec2 f_pos;
out float f_radius;
out vec4 f_rgba;

void main() {
    // do camera transforms
    vec2 cam_relative = (pos - cam_pos) * cam_zoom;
    vec2 cam_center = (center - cam_pos) * cam_zoom;

    // pass data to fragment shader
    f_center = cam_center;
    f_pos = cam_relative;
    f_radius = radius * cam_zoom;
    f_rgba = rgba;

    // prevent stretching with window resize
    cam_relative.x *= inverse_aspect;

    // write to glPosition
    gl_Position = vec4(cam_relative, 0.0, 1.0);
}
