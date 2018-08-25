/*
* Basic Blinn-Phong shading using material attributes.
* This is the default shader for all models.
* TODO textures
*/
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) out vec4 target;

layout (location = 0) in vec3 v_position;
layout (location = 1) in vec3 v_normal;

uniform LightBlock {
    vec3 light_dir;
};

const vec3 ambient_color = vec3(0.4, 0.4, 0.4);
const vec3 diffuse_color = vec3(0.8, 0.8, 0.8); // Light grey
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
    vec3 light_dir_normalized = normalize(light_dir);
    vec3 v_normal_normalized = normalize(v_normal);
    float diffuse = max(dot(v_normal_normalized, light_dir_normalized), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(light_dir_normalized + camera_dir);
    float specular = pow(max(dot(half_direction, v_normal_normalized), 0.0), 16.0);

    target = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}