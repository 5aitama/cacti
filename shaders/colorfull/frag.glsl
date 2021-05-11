#version 330 core

uniform vec2 screen_resolution;
uniform float time;

in vec2 out_uv;
out vec4 FragColor;

void main() 
{
    vec2 uv = (out_uv.xy - 0.5 * screen_resolution.xy) / screen_resolution.y;
    vec3 o_color = 0.5 + 0.5 * cos(time + uv.xyx + vec3(0, 2, 4));

    FragColor = vec4(o_color, 1.0);
}