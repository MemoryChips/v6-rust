// Simple Shader

#type vertex
// #version 330 core
#version 150
in vec2 position;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}

#type fragment
// #version 330 core
#version 150
out vec4 out_color;
void main() {
    out_color = vec4(1.0, 1.0, 0.0, 1.0);
}

