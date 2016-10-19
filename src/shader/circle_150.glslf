#version 150 core

in vec3 v_Color;

out vec4 Target0;

void main() {
  float dist = distance(2.0 * gl_PointCoord - 1.0, vec2(0.0, 0.0));
  float delta = fwidth(dist);
  float alpha = 1.0 - smoothstep(1.0 - delta, 1.0 + delta, dist);
  Target0 = vec4(v_Color, 1.0) * alpha;
}
