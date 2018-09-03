/*
* Basic fragment shader using Blinn-Phong
* shading. TODO textures
*/
#version 320

out vec4 target;

in vec3 v_position;
in vec3 v_normal;

const vec3 light_dir = vec3(0.0, 0.0, 1.0);
const vec3 ambient_color = vec3(0.0, 0.0, 0.3);
const vec3 diffuse_color = vec3(0.0, 0.0, 1.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
    float diffuse = max(dot(normalize(v_normal), normalize(light_dir)), 0.0);

    // This is a hack to get specular lighting to work
    // correctly. In the Vulkan backend, Y coordinates
    // are inverted, so we need to make some modifications
    // for this to work in the OpenGL backend. Therefore,
    // we invert the Y coordinate of the passed position,
    // which for some reason works.
    vec3 v_position_real = vec3(v_position.x, -v_position.y, v_position.z);

    vec3 camera_dir = normalize(-v_position_real);

    vec3 half_direction = normalize(normalize(light_dir) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    target = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}