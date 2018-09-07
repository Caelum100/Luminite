/*
* Basic fragment shader using Blinn-Phong
* shading. TODO textures
*/
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) out vec4 target;

layout (location = 0) in vec3 v_position;
layout (location = 1) in vec3 v_normal;

const vec3 light_dir = vec3(0.0, 0.0, 1.0);
const vec3 ambient_color = vec3(0.0, 0.0, 0.3);
const vec3 diffuse_color = vec3(0.0, 0.0, 1.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
    float diffuse = max(abs(dot(normalize(v_normal), normalize(light_dir))), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(light_dir) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    target = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}