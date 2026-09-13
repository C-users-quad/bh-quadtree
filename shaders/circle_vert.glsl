#version 330

in vec2 unit_pos;

in vec2 i_center;
in float i_radius;
in vec4 i_rgba;

uniform vec2 cam_pos;
uniform float cam_zoom;
uniform float inverse_aspect;
uniform vec2 screen_size;

out vec2 f_local;   // local space position for discard test
out float f_pixel_size;
out vec4 f_rgba;

#define ROOT2 1.4142135

void main() {
    float screen_height = screen_size.y;

    vec2 position = i_center;
    float radius = i_radius;

    // if particle would be smaller than ~1px, snap to pixel grid and clamp radius
    float min_radius = ROOT2 * (1.0 / cam_zoom) / screen_height;
    if (radius < min_radius) {
        // snap center to nearest pixel
        vec2 relative = (position - cam_pos) * cam_zoom;
        vec2 nearest_pixel = (floor(relative * screen_height) + 0.5) / screen_height;
        position = nearest_pixel / cam_zoom + cam_pos;
        radius = min_radius;
    }

    vec2 world_pos = position + unit_pos * radius;
    vec2 cam_relative = (world_pos - cam_pos) * cam_zoom;

    f_local = unit_pos;  // [-1,1] local coords, length > 1 means outside circle
    f_pixel_size = 1.0 / (radius * cam_zoom * screen_height);
    f_rgba = i_rgba;

    cam_relative.x *= inverse_aspect;
    gl_Position = vec4(cam_relative, 0.0, 1.0);
}
