#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Texture;
uniform Locals {
  mat4 u_Proj;
  float u_Strength;
};

out vec4 Target0;

void main() {
  vec2 uv0 = v_Uv + vec2(-u_Strength, -u_Strength);
  vec2 uv1 = v_Uv + vec2(0, -u_Strength);
  vec2 uv2 = v_Uv + vec2(u_Strength, -u_Strength);
  vec2 uv3 = v_Uv + vec2(-u_Strength, 0);
  vec2 uv4 = v_Uv + vec2(0, 0);
  vec2 uv5 = v_Uv + vec2(u_Strength, 0);
  vec2 uv6 = v_Uv + vec2(-u_Strength, u_Strength);
  vec2 uv7 = v_Uv + vec2(0, u_Strength);
  vec2 uv8 = v_Uv + vec2(u_Strength, u_Strength);

  vec4 t0 = texture(t_Texture, uv0);
  vec4 t1 = texture(t_Texture, uv1);
  vec4 t2 = texture(t_Texture, uv2);
  vec4 t3 = texture(t_Texture, uv3);
  vec4 t4 = texture(t_Texture, uv4);
  vec4 t5 = texture(t_Texture, uv5);
  vec4 t6 = texture(t_Texture, uv6);
  vec4 t7 = texture(t_Texture, uv7);
  vec4 t8 = texture(t_Texture, uv8);

  Target0 = (1 * t0 + 2 * t1 + 1 * t2 +
    2 * t3 + 4 * t4 + 2 * t5 +
    1 * t6 + 2 * t7 + 1 * t8) / 16.0;
}
