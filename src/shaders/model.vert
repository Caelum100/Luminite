/*
* Vertex shader using an MVP matrix
* to calculate vertex positions.
* This is the default shader for all models.
*/
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

layout (location = 0) out vec3 v_position;
layout (location = 1) out vec3 v_normal;

uniform MatrixBlock {
    mat4 matrix;
    mat4 modelview; // Redundunt, but needed for fragment shader
};

void main() {
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = matrix * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}