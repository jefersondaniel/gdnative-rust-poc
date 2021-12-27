shader_type canvas_item;
render_mode blend_mix;

uniform sampler2D palette;

void fragment() {
    float r = texture(TEXTURE, UV).r;
    ivec2 pallete_size = textureSize(palette, 0);
    vec4 palette_color = texture(palette, vec2(r * 255.0 / float(pallete_size.x), 0), 0);
    COLOR = vec4(palette_color.r, palette_color.g, palette_color.b, palette_color.a);
}
