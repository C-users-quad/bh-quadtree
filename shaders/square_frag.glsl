#version 330

in vec4 f_rgba;
out vec4 p_rgba;

void main() {
    // just output the color given. nothing fancy.
    p_rgba = f_rgba;
}
