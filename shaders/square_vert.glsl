#version 330

in vec2 pos;
in vec4 rgba;

uniform vec2 cam_pos;
uniform float cam_zoom;
uniform float inverse_aspect;

out vec4 f_rgba;

void main() {
    // do camera transforms
    vec2 cam_relative = (pos - cam_pos) * cam_zoom;
    // prevent stretching with window resize
    cam_relative.x *= inverse_aspect;

    // write to gl_Position
    gl_Position = vec4(cam_relative, 0.0, 1.0);
    // pass color to fragment shader
    f_rgba = rgba;
}
