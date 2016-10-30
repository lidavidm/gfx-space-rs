#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Texture;

out vec4 Target0;

void main() {
  Target0 = 0.4 * texture(t_Texture, v_Uv) +
    0.15 * texture(t_Texture, v_Uv - vec2(0.005, 0)) +
    0.15 * texture(t_Texture, v_Uv + vec2(0.005, 0)) +
    0.15 * texture(t_Texture, v_Uv - vec2(0, 0.005)) +
    0.15 * texture(t_Texture, v_Uv + vec2(0, 0.005));
}
