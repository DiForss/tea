#version 330 core

in vec4 v_Color;
out vec4 Target;

void main() {
	Target = v_Color;
}