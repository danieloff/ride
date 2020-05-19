#version 330 core

out vec4 FragColor;

uniform samplerCube cube_beach;

uniform vec3 camera_direction;

in vec3 fNormal;

void main() {
  //gl_FragColor = vec4((fNormal + 1.0) / 2.0, 1.0);
  FragColor = texture(cube_beach, reflect(camera_direction, fNormal));
}
