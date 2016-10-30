#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Texture;

out vec4 Target0;

void main() {
  Target0 = texture(t_Texture, v_Uv);
  // Target0 = vec4(0.0, 0.0, 0.0, 1.0);
}
