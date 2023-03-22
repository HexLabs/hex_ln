#version 460
layout(location=0) in vec3 rgb;

out vec4 frag_color;

void main() {
	frag_color = vec4(rgb, 1.0);
}
