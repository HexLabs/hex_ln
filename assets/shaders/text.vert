#version 460
layout(location=0) in vec2 pos;
layout(location=1) in vec2 tex;

out vec2 tex_coords;

layout(location=0) uniform mat4 projection;

void main() {
	tex_coords = tex;
	gl_Position = projection * vec4(pos, 0.0, 1.0);
}
