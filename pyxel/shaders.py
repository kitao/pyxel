DRAWING_VERTEX_SHADER = """
#version 120

#define unpack_4ui_1(x) int(mod(x / 0x1, 0x10));
#define unpack_4ui_2(x) int(mod(x / 0x10, 0x10));
#define unpack_4ui_3(x) int(mod(x / 0x100, 0x10));
#define unpack_4ui_4(x) int(mod(x / 0x1000, 0x10));

const int TYPE_PIX = 0;
const int TYPE_LINE = 1;
const int TYPE_RECT = 2;
const int TYPE_RECTB = 3;
const int TYPE_CIRC = 4;
const int TYPE_CIRCB = 5;
const int TYPE_BLT = 6;

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
{
    return vec4(pos * 2.0 / u_framebuffer_size - 1.0, 0.0, 1.0);
}

void pix()
{
    gl_PointSize = 1.0;
    gl_Position = pixelToScreen(a_pos.xy);
}

void line()
{
    v_pos1 = a_pos.xy;
    v_pos2 = a_pos.zw;

    v_min_pos = min(a_pos.xy, a_pos.zw);
    v_max_pos = max(a_pos.xy, a_pos.zw);

    vec2 diff = v_max_pos - v_min_pos;

    gl_PointSize = max(diff.x, diff.y) + 1.0;
    gl_Position = pixelToScreen(v_min_pos + (gl_PointSize - 1.0) * 0.5);
}

void rect_rectb()
{
    v_min_pos = a_pos.xy;
    v_max_pos = a_pos.xy + a_size - 1.0;

    v_size = a_size;

    gl_PointSize = max(a_size.x, a_size.y);
    gl_Position = pixelToScreen(v_min_pos + (gl_PointSize - 1.0) * 0.5);
}

void circ_circb()
{
    v_pos1 = a_pos.xy;
    v_size.x = a_size.x;

    gl_PointSize = v_size.x * 2.0 + 1.0;
    gl_Position = pixelToScreen(v_pos1);
}

void blt()
{
    vec2 abs_size = abs(a_size);

    v_pos1 = a_pos.xy;
    v_pos2 = a_pos.zw;

    v_min_pos = v_pos1;
    v_max_pos = v_pos1 + abs_size;

    v_size = a_size;

    gl_PointSize = max(abs_size.x, abs_size.y);
    gl_Position = pixelToScreen(v_min_pos + (gl_PointSize - 1.0) * 0.5);
}

void main()
{
    v_type = a_mode.x;
    v_col = a_mode.y;
    v_image = a_mode.z;

    v_min_clip = a_clip.xy;
    v_max_clip = a_clip.xy + a_clip.zw - 1.0;

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

    if (v_type == TYPE_PIX) { pix(); }
    else if (v_type == TYPE_LINE) { line(); }
    else if (v_type == TYPE_RECT || v_type == TYPE_RECTB) { rect_rectb(); }
    else if (v_type == TYPE_CIRC || v_type == TYPE_CIRCB) { circ_circb(); }
    else if (v_type == TYPE_BLT) { blt(); }
    else { gl_Position = vec4(0.0, 0.0, 0.0, 1.0); }
}
"""

DRAWING_FRAGMENT_SHADER = """
#version 120

const int TYPE_PIX = 0;
const int TYPE_LINE = 1;
const int TYPE_RECT = 2;
const int TYPE_RECTB = 3;
const int TYPE_CIRC = 4;
const int TYPE_CIRCB = 5;
const int TYPE_BLT = 6;

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

vec2 pos;

vec4 indexToColor(float col)
{
    return vec4(u_palette[int(v_pal[int(col)])] / 255.0, 1.0);
}

void pix()
{
    gl_FragColor = indexToColor(v_col);
}

void line()
{
    if (pos.x < v_min_pos.x || pos.y < v_min_pos.y ||
        pos.x > v_max_pos.x || pos.y > v_max_pos.y) { discard; }

    if (v_pos1.x == v_pos2.x)
    {
        if (pos.x != v_pos1.x) { discard; }
    }
    else if (v_pos1.y == v_pos2.y)
    {
        if (pos.y != v_pos1.y) { discard; }
    }
    else
    {
        vec2 diff = v_pos2 - v_pos1;

        if (abs(diff.x) > abs(diff.y))
        {
            float y = v_pos1.y + diff.y * abs((pos.y - v_pos1.y) / diff.x);
            if (pos.y > int(y)) { discard; }
        }
        else
        {
            float x = v_pos1.x + diff.x * abs((pos.x - v_pos1.x) / diff.y);
            if (pos.x > int(x)) { discard; }
        }
    }

    gl_FragColor = indexToColor(v_col);
}

void rect()
{
    if (pos.x < v_min_pos.x || pos.y < v_min_pos.y ||
        pos.x > v_max_pos.x || pos.y > v_max_pos.y) { discard; }

    gl_FragColor = indexToColor(v_col);
}

void rectb()
{
    if (pos.x != v_min_pos.x && pos.y != v_min_pos.y &&
        pos.x != v_max_pos.x && pos.y != v_max_pos.y) { discard; }

    if (pos.x < v_min_pos.x || pos.y < v_min_pos.y ||
        pos.x > v_max_pos.x || pos.y > v_max_pos.y) { discard; }

    gl_FragColor = indexToColor(v_col);
}

void circ()
{
    float dist = distance(pos, v_pos1);

    if (dist > v_size.x + 0.41) { discard; }

    gl_FragColor = indexToColor(v_col);
}

void circb()
{
    float dist = distance(pos, v_pos1);

    if (dist > v_size.x + 0.4 || dist < v_size.x + 0.4 - 0.8) { discard; }

    gl_FragColor = indexToColor(v_col);
}

void blt()
{
    int tex_no = int(v_image);
    vec2 uv = (v_pos2 + pos - v_min_pos) / u_texture_size[tex_no];
    int col = int(texture2D(u_texture[tex_no], uv).r * 255.0);

    if (col == int(v_col)) { discard; }

    gl_FragColor = indexToColor(col);
}

void main()
{
    pos = floor(gl_FragCoord.xy);

    if (pos.x < v_min_clip.x || pos.y < v_min_clip.y ||
        pos.x > v_max_clip.x || pos.y > v_max_clip.y) { discard; }

    if (v_type == TYPE_PIX) { pix(); }
    else if (v_type == TYPE_LINE) { line(); }
    else if (v_type == TYPE_RECT) { rect(); }
    else if (v_type == TYPE_RECTB) { rectb(); }
    else if (v_type == TYPE_CIRC) { circ(); }
    else if (v_type == TYPE_CIRCB) { circb(); }
    else if (v_type == TYPE_BLT) { blt(); }
    else { gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0); }
}
"""

DRAWING_ATTRIBUTE_INFO = [
    ('a_mode', 0, 3),
    ('a_pos', 3, 4),
    ('a_size', 7, 2),
    ('a_clip', 9, 4),
    ('a_pal', 13, 4),
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

SCALING_ATTRIBUTE_INFO = [
    ('a_pos', 0, 2),
    ('a_uv', 2, 2),
]
