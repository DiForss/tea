#version 330 core

in vec4 a_Pos;
in vec3 a_Color;

out vec4 v_Color;

void main() {
	v_Color = vec4(a_Color, 1.0);
	gl_Position = a_Pos;
}