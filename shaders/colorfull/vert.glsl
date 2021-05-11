#version 330 core

layout (location = 0) in vec2 in_pos;
layout (location = 1) in vec2 in_norm;
layout (location = 2) in vec2 in_uv;

out vec2 out_uv;

void main() {
    out_uv = in_uv;
    gl_Position = vec4(in_pos, 1.0, 1.0);
}