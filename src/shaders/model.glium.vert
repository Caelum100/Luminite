/*
* Vertex shader using an MVP matrix
* to calculate vertex positions.
* This is the default shader for all models.
*
* Because glium lacks support for proper uniforms,
* we need a separate shader to use glium.
*/
#version 320

in vec3 a_position;
in vec3 a_normal;

out vec3 v_position;
out vec3 v_normal;

uniform mat4 matrix;
uniform mat4 modelview; // TODO - this is redundant

void main() {
    gl_Position = matrix * vec4(a_position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
    v_normal = transpose(inverse(mat3(modelview))) * a_normal;
}