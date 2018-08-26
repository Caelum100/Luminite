/*
* Basic fragment shader using colors for
* each vertex. TODO textures / Blinn-Phong
*/
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) out vec4 target;

layout (location = 0) in vec3 v_color;

void main() {
    target = vec4(v_color, 1.0);
}