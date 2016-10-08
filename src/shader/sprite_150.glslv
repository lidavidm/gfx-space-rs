#version 150 core

in vec2 a_Pos;
in vec3 a_Color;
in vec2 a_Uv;

out vec4 v_Color;
out vec2 v_Uv;

uniform Locals {
  mat4 u_Proj;
  mat4 u_View;
  mat4 u_Model;
};

void main() {
  v_Color = vec4(a_Color, 1.0);
  v_Uv = a_Uv;
  gl_Position = u_Proj * u_View * u_Model * vec4(a_Pos, 0.0, 1.0);
}
