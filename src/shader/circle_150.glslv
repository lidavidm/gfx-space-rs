#version 150 core

in vec2 a_Pos;

out vec3 v_Color;

uniform Locals {
  mat4 u_Proj;
  mat4 u_View;
  mat4 u_Model;
  vec3 u_Color;
};

void main() {
  v_Color = u_Color;
  gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 0.0, 1.0);
}
