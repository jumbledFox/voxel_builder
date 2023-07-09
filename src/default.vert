#version 140

in vec3 position;
in uint tex_coords;
in uint light_level;
in uint texture_id;
in float ambient_occlusion;

out vec2 v_tex_coords;
out float v_light_level;
out vec3 v_chunk_colour;
out float v_texture_id;
out float v_ambient_occlusion;

uniform ivec3 chunk_position;
uniform uint chunk_colour;
uniform bool colour_chunks;
uniform mat4 perspective;
uniform mat4 view;
uniform mat4 matrix;

vec2 tex_coords_array[4] = vec2[4](
	vec2(0, 1),
	vec2(1, 1),
	vec2(1, 0),
	vec2(0, 0)
);

vec3 chunk_colour_array[5] = vec3[5](
	vec3(0.8, 0.3, 0.3),
	vec3(0.8, 0.8, 0.3),
	vec3(0.3, 0.8, 0.3),
	vec3(0.5, 0.8, 1.0),
	vec3(1.0, 1.0, 1.0)
);

void main() {
	v_texture_id = float(texture_id);
	v_ambient_occlusion = 0.2*(1-(ambient_occlusion/3));
    v_tex_coords = tex_coords_array[tex_coords];
	if (colour_chunks) {
		v_chunk_colour = chunk_colour_array[(chunk_colour%5u)];
	} else {
		v_chunk_colour = vec3(1.0, 1.0, 1.0);
	}
    v_light_level = float(light_level) / 5;
    gl_Position = perspective * (view * matrix) * vec4((position + (chunk_position * 32)), 1.0);
}