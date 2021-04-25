#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 in_uv;

uniform mat4 transform;
uniform mat4 projection;

out vec2 out_uv;

void main() {
    out_uv = in_uv;
    gl_Position = projection * transform * vec4(aPos.xyz, 1.0);
}