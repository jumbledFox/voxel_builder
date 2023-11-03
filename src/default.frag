#version 140
#extension GL_EXT_texture_array : enable

in vec2 v_tex_coords;
in float v_light_level;
in vec3 v_chunk_colour; 
in float v_texture_id;
in float v_ambient_occlusion;

out vec4 color;

uniform sampler2DArray texture_array;

uniform uint draw_mode;

void main() {
    //color = vec4(1.0, 1.0, 1.0, 1.0);
    //color = texture(tex, v_tex_coords) * vec4(v_chunk_colour, 1) * vec4(vec3(v_light_level), 1);

    if (draw_mode == 0u) { // All
        color = texture2DArray(texture_array, vec3(v_tex_coords, v_texture_id)) * vec4(v_chunk_colour, 1) * vec4(vec3(v_light_level - v_ambient_occlusion), 1);
    } else if (draw_mode == 1u) { // AO and face lighting
        color = vec4(v_chunk_colour, 1) * vec4(vec3(v_light_level - v_ambient_occlusion), 1);
    } else { // AO
        color = vec4(v_chunk_colour, 1) * vec4(vec3(1 - (v_ambient_occlusion*2.2)), 1);
    }
    if (color.a == 0) {
        discard;
    }
}