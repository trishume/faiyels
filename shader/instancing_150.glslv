#version 150 core

in vec2 a_Position;
in vec2 a_Translate;
in uint a_Color;

uniform b_VsLocals {
  mat4 u_Transform;
  mat4 u_Proj;
};

out vec4 v_Color;

void main() {
    gl_Position = u_Proj*u_Transform*vec4(a_Position + a_Translate, 0.0, 1.0);

    uint u8mask = 0x000000FFu;
    v_Color = vec4(float( a_Color >> 24),
                   float((a_Color >> 16) & u8mask),
                   float((a_Color >>  8) & u8mask),
                   float( a_Color        & u8mask)) / 255.0;
}
