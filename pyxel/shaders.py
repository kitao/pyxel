from .constants import (
    DRAW_TYPE_BLT,
    DRAW_TYPE_CIRC,
    DRAW_TYPE_CIRCB,
    DRAW_TYPE_LINE,
    DRAW_TYPE_PIX,
    DRAW_TYPE_RECT,
    DRAW_TYPE_RECTB,
    DRAW_TYPE_TEXT,
    FONT_HEIGHT,
    FONT_ROW_COUNT,
    FONT_WIDTH,
    RENDERER_IMAGE_COUNT,
)

DRAWING_VERTEX_SHADER = """
#version 120

#define unpack_4ui_1(x) int(mod(x / 0x1, 0x10))
#define unpack_4ui_2(x) int(mod(x / 0x10, 0x10))
#define unpack_4ui_3(x) int(mod(x / 0x100, 0x10))
#define unpack_4ui_4(x) int(mod(x / 0x1000, 0x10))

const int TYPE_PIX = {1};
const int TYPE_LINE = {2};
const int TYPE_RECT = {3};
const int TYPE_RECTB = {4};
const int TYPE_CIRC = {5};
const int TYPE_CIRCB = {6};
const int TYPE_BLT = {7};
const int TYPE_TEXT = {8};

uniform vec2 u_framebuffer_size;

attribute vec3 a_mode;
attribute vec4 a_pos;
attribute vec2 a_size;
attribute vec4 a_clip;
attribute vec4 a_pal;

varying float v_type;
varying float v_col;
varying float v_image;
varying vec2 v_pos1;
varying vec2 v_pos2;
varying vec2 v_min_pos;
varying vec2 v_max_pos;
varying vec2 v_size;
varying vec2 v_min_clip;
varying vec2 v_max_clip;
varying float v_pal[16];

vec4 pixelToScreen(vec2 pos)
{{
    return vec4((pos + 0.5 + u_framebuffer_size) * 2.0 / (u_framebuffer_size * 3.0) - 1.0, 0.0, 1.0);
}}

void pix()
{{
    vec2 p = floor(a_pos.xy + 0.5);

    v_min_pos = v_max_pos = p;

    gl_PointSize = 1.0;
    gl_Position = pixelToScreen(p);
}}

void line()
{{
    vec2 p1 = floor(a_pos.xy + 0.5);
    vec2 p2 = floor(a_pos.zw + 0.5);

    v_min_pos = min(p1, p2);
    v_max_pos = max(p1, p2);

    vec2 d = v_max_pos - v_min_pos;

    if (d.x > d.y)
    {{
        if (p1.x < p2.x) {{
            v_pos1 = p1;
            v_pos2 = p2;
        }}
        else
        {{
            v_pos1 = p2;
            v_pos2 = p1;
        }}
    }}
    else
    {{
        if (p1.y < p2.y)
        {{
            v_pos1 = p1;
            v_pos2 = p2;
        }}
        else
        {{
            v_pos1 = p2;
            v_pos2 = p1;
        }}
    }}

    gl_PointSize = max(d.x, d.y) + 1.0;
    gl_Position = pixelToScreen(v_min_pos + (gl_PointSize - 1.0) * 0.5);
}}

void rect_rectb()
{{
    vec2 p1 = floor(a_pos.xy + 0.5);
    vec2 p2 = floor(a_pos.zw + 0.5);

    v_min_pos = min(p1, p2);
    v_max_pos = max(p1, p2);

    vec2 s = v_max_pos - v_min_pos + 1.0;

    gl_PointSize = max(s.x, s.y);
    gl_Position = pixelToScreen(v_min_pos + (gl_PointSize - 1.0) * 0.5);
}}

void circ_circb()
{{
    vec2 p = floor(a_pos.xy + 0.5);
    float r = floor(a_size.x + 0.5);

    v_pos1 = p;
    v_min_pos = p - r;
    v_max_pos = p + r;
    v_size.x = r;

    gl_PointSize = r * 2.0 + 1.0;
    gl_Position = pixelToScreen(p);
}}

void blt()
{{
    vec2 p1 = floor(a_pos.xy + 0.5);
    vec2 p2 = floor(a_pos.zw + 0.5);
    vec2 s = floor(a_size + 0.5);
    vec2 abs_s = abs(s);

    v_pos1 = p1;
    v_pos2 = p2;
    v_min_pos = p1;
    v_max_pos = p1 + abs_s - 1.0;
    v_size = s;

    gl_PointSize = max(abs_s.x, abs_s.y);
    gl_Position = pixelToScreen(v_min_pos + (gl_PointSize - 1.0) * 0.5);
}}

void text()
{{
    v_image = {0} - 1;
    vec2 p1 = floor(a_pos.xy + 0.5);
    vec2 p2 = vec2(mod(a_pos.z, {11}) * {9}, floor(a_pos.z / {11}) * {10});
    vec2 s = vec2({9}, {10});
    vec2 abs_s = abs(s);

    v_pos1 = p1;
    v_pos2 = p2;
    v_min_pos = p1;
    v_max_pos = p1 + abs_s - 1.0;
    v_size = s;

    gl_PointSize = max(abs_s.x, abs_s.y);
    gl_Position = pixelToScreen(v_min_pos + (gl_PointSize - 1.0) * 0.5);
}}

void main()
{{
    v_type = a_mode.x;
    v_col = a_mode.y;
    v_image = a_mode.z;

    v_pal[0] = unpack_4ui_1(a_pal.x);
    v_pal[1] = unpack_4ui_2(a_pal.x);
    v_pal[2] = unpack_4ui_3(a_pal.x);
    v_pal[3] = unpack_4ui_4(a_pal.x);
    v_pal[4] = unpack_4ui_1(a_pal.y);
    v_pal[5] = unpack_4ui_2(a_pal.y);
    v_pal[6] = unpack_4ui_3(a_pal.y);
    v_pal[7] = unpack_4ui_4(a_pal.y);
    v_pal[8] = unpack_4ui_1(a_pal.z);
    v_pal[9] = unpack_4ui_2(a_pal.z);
    v_pal[10] = unpack_4ui_3(a_pal.z);
    v_pal[11] = unpack_4ui_4(a_pal.z);
    v_pal[12] = unpack_4ui_1(a_pal.w);
    v_pal[13] = unpack_4ui_2(a_pal.w);
    v_pal[14] = unpack_4ui_3(a_pal.w);
    v_pal[15] = unpack_4ui_4(a_pal.w);

    if (v_type == TYPE_PIX) {{ pix(); }}
    else if (v_type == TYPE_LINE) {{ line(); }}
    else if (v_type == TYPE_RECT || v_type == TYPE_RECTB) {{ rect_rectb(); }}
    else if (v_type == TYPE_CIRC || v_type == TYPE_CIRCB) {{ circ_circb(); }}
    else if (v_type == TYPE_BLT) {{ blt(); }}
    else if (v_type == TYPE_TEXT) {{ text(); }}
    else {{ gl_Position = vec4(0.0, 0.0, 0.0, 1.0); }}

    vec2 p1 = floor(a_clip.xy + 0.5);
    vec2 p2 = floor(a_clip.zw + 0.5);

    v_min_clip = max(min(p1, p2), v_min_pos);
    v_max_clip = min(max(p1, p2), v_max_pos);
}}
""".format(
    RENDERER_IMAGE_COUNT,
    DRAW_TYPE_PIX,
    DRAW_TYPE_LINE,
    DRAW_TYPE_RECT,
    DRAW_TYPE_RECTB,
    DRAW_TYPE_CIRC,
    DRAW_TYPE_CIRCB,
    DRAW_TYPE_BLT,
    DRAW_TYPE_TEXT,
    FONT_WIDTH,
    FONT_HEIGHT,
    FONT_ROW_COUNT,
)

DRAWING_FRAGMENT_SHADER = """
#version 120

const int TYPE_PIX = {0};
const int TYPE_LINE = {1};
const int TYPE_RECT = {2};
const int TYPE_RECTB = {3};
const int TYPE_CIRC = {4};
const int TYPE_CIRCB = {5};
const int TYPE_BLT = {6};
const int TYPE_TEXT = {7};

uniform ivec3 u_palette[16];
uniform sampler2D u_texture[8];
uniform vec2 u_texture_size[8];

varying float v_type;
varying float v_col;
varying float v_image;
varying vec2 v_pos1;
varying vec2 v_pos2;
varying vec2 v_min_pos;
varying vec2 v_max_pos;
varying vec2 v_size;
varying vec2 v_min_clip;
varying vec2 v_max_clip;
varying float v_pal[16];

vec2 draw_pos;

vec4 indexToColor(float col)
{{
    return vec4(u_palette[int(v_pal[int(col)])] / 255.0, 1.0);
}}

void pix()
{{
    gl_FragColor = indexToColor(v_col);
}}

void line()
{{
    vec2 d = v_max_pos - v_min_pos;

    if (d.x != 0.0 || d.y != 0.0)
    {{
        if (d.x > d.y)
        {{
            float a = (v_pos2.y - v_pos1.y) / d.x;
            float y = floor((draw_pos.x - v_pos1.x) * a + v_pos1.y + 0.5);

            if (draw_pos.y != y) {{ discard; }}
        }}
        else
        {{
            float a = (v_pos2.x - v_pos1.x) / d.y;
            float x = floor((draw_pos.y - v_pos1.y) * a + v_pos1.x + 0.5);

            if (draw_pos.x != x) {{ discard; }}
        }}
    }}

    gl_FragColor = indexToColor(v_col);
}}

void rect()
{{
    gl_FragColor = indexToColor(v_col);
}}

void rectb()
{{
    if (draw_pos.x != v_min_pos.x && draw_pos.x != v_max_pos.x &&
        draw_pos.y != v_min_pos.y && draw_pos.y != v_max_pos.y) {{ discard; }}

    gl_FragColor = indexToColor(v_col);
}}

void circ()
{{
    vec2 d = abs(draw_pos - v_pos1);

    if (d.x > d.y)
    {{
        float x = floor(sqrt(v_size.x * v_size.x - d.y * d.y) + 0.5);
        if (d.x > x) {{ discard; }}
    }}
    else
    {{
        float y = floor(sqrt(v_size.x * v_size.x - d.x * d.x) + 0.5);
        if (d.y > y) {{ discard; }}
    }}

    gl_FragColor = indexToColor(v_col);
}}

void circb()
{{
    vec2 d = abs(draw_pos - v_pos1);

    if (d.x > d.y)
    {{
        float x = floor(sqrt(v_size.x * v_size.x - d.y * d.y) + 0.5);
        if (d.x != x) {{ discard; }}
    }}
    else
    {{
        float y = floor(sqrt(v_size.x * v_size.x - d.x * d.x) + 0.5);
        if (d.y != y) {{ discard; }}
    }}

    gl_FragColor = indexToColor(v_col);
}}

#define texture_color(index) \\
    int(texture2D(u_texture[index], uv).r * 255.0 + 0.5)

int getTextureColor(int index, vec2 uv)
{{
    if (index == 0) {{ return texture_color(0); }}
    else if (index == 1) {{ return texture_color(1); }}
    else if (index == 2) {{ return texture_color(2); }}
    else if (index == 3) {{ return texture_color(3); }}
    else if (index == 4) {{ return texture_color(4); }}
    else if (index == 5) {{ return texture_color(5); }}
    else if (index == 6) {{ return texture_color(6); }}
    else if (index == 7) {{ return texture_color(7); }}
}}

void blt()
{{
    int img = int(v_image);
    vec2 p = draw_pos - v_min_pos;
    vec2 uv = v_pos2;
    uv.x += (v_size.x > 0.0) ? p.x : -(v_size.x + 1.0 + p.x);
    uv.y += (v_size.y > 0.0) ? p.y : -(v_size.y + 1.0 + p.y);
    uv /= u_texture_size[img];

    int col = getTextureColor(img, uv);
    if (col == v_col) {{ discard; }}

    gl_FragColor = indexToColor(col);
}}

void text()
{{
    int img = int(v_image);
    vec2 uv = (v_pos2 + draw_pos - v_min_pos) / u_texture_size[img];

    int col = getTextureColor(img, uv);
    if (col == 0) {{ discard; }}

    gl_FragColor = indexToColor(v_col);
}}

void main()
{{
    draw_pos = floor(gl_FragCoord.xy);

    if (draw_pos.x < v_min_clip.x || draw_pos.y < v_min_clip.y ||
        draw_pos.x > v_max_clip.x || draw_pos.y > v_max_clip.y) {{ discard; }}

    if (v_type == TYPE_PIX) {{ pix(); }}
    else if (v_type == TYPE_LINE) {{ line(); }}
    else if (v_type == TYPE_RECT) {{ rect(); }}
    else if (v_type == TYPE_RECTB) {{ rectb(); }}
    else if (v_type == TYPE_CIRC) {{ circ(); }}
    else if (v_type == TYPE_CIRCB) {{ circb(); }}
    else if (v_type == TYPE_BLT) {{ blt(); }}
    else if (v_type == TYPE_TEXT) {{ text(); }}
    else {{ gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0); }}
}}
""".format(
    DRAW_TYPE_PIX,
    DRAW_TYPE_LINE,
    DRAW_TYPE_RECT,
    DRAW_TYPE_RECTB,
    DRAW_TYPE_CIRC,
    DRAW_TYPE_CIRCB,
    DRAW_TYPE_BLT,
    DRAW_TYPE_TEXT,
)

DRAWING_ATTRIBUTE_INFO = [
    ("a_mode", 0, 3),
    ("a_pos", 3, 4),
    ("a_size", 7, 2),
    ("a_clip", 9, 4),
    ("a_pal", 13, 4),
]

SCALING_VERTEX_SHADER = """
#version 120

attribute vec2 a_pos;
attribute vec2 a_uv;

varying vec2 v_uv;

void main()
{
    v_uv = a_uv;

    gl_Position = vec4(a_pos, 0.0, 1.0);
}
"""

SCALING_FRAGMENT_SHADER = """
#version 120

uniform sampler2D u_texture;

varying vec2 v_uv;

void main()
{
    gl_FragColor = texture2D(u_texture, v_uv);
}
"""

SCALING_ATTRIBUTE_INFO = [("a_pos", 0, 2), ("a_uv", 2, 2)]
