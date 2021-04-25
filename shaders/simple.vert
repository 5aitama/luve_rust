#version 330 core

layout (location = 0) in vec3 aPos;

uniform float time;
uniform mat4 transform;
uniform mat4 projection;

void main() {
    gl_Position = projection * transform * vec4(aPos.xyz, 1.0);
}