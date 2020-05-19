#version 330 core

//attribute vec3 vPosition;
//attribute vec3 vNormal;
layout (location = 0) in vec3 vPosition;
layout (location = 1) in vec3 vNormal;

uniform mat4 world;
uniform mat4 view;
uniform mat4 proj;

out vec3 fNormal;

void main() {
  fNormal = vNormal;
  gl_Position = proj * view * world * vec4(vPosition, 1);
}
