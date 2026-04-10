#version 330

in vec2 unit_pos;

in float i_left;
in float i_top;
in float i_size;
in vec4 i_rgba;

uniform vec2 cam_pos;
uniform float cam_zoom;
uniform float inverse_aspect;
uniform vec2 screen_size; 

out vec4 f_rgba;

void main() {
    // unit_pos is in [-1,1], remap to [0,1] then scale to node bounds
    vec2 local = (unit_pos * 0.5 + 0.5);  // [0,1]
    vec2 world_pos = vec2(i_left, i_top) + local * i_size;

    vec2 cam_relative = (world_pos - cam_pos) * cam_zoom;
    cam_relative.x *= inverse_aspect;

    gl_Position = vec4(cam_relative, 0.0, 1.0);
    f_rgba = i_rgba;
}
