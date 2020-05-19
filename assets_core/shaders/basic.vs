#version 330 core

in vec4 in_vertex;
in vec3 in_normal;

uniform mat4 world_matrix;
uniform mat4 proj_matrix;
uniform mat4 view_matrix;

out vec3 normal;

void main() {
  normal = in_normal;
  gl_Position = proj_matrix * view_matrix * world_matrix * in_vertex;
}
