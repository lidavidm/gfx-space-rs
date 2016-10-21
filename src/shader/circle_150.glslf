#version 150 core

in vec3 v_Color;

out vec4 Target0;

void main() {
  vec2 cxy = 2.0 * gl_PointCoord - 1.0;
  float dist = dot(cxy, cxy);
  float delta = fwidth(dist);
  float alpha = 1.0 - smoothstep(1.0 - delta, 1.0 + delta, dist);
  Target0 = vec4(v_Color, alpha);
}
