/*
* Vertex shader using an MVP matrix
* to calculate vertex positions
* This is the default shader for all models.
*/
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

out vec3 v_position;
out vec3 v_normal;

uniform mat4 matrix;
uniform mat4 view;
uniform mat4 model; // Redundunt, but needed for fragment shader

void main() {
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = matrix * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}