#define PI 3.14159265359
#define sin01(x) sin((x)*PI*2.)
#define cos01(x) cos((x)*PI*2.)
#define hump(x) cos(clamp(x,-.5,.5)*PI)
#define humpW(x,w) cos(clamp((x)/(w),-.5,.5)*PI)
#define hump01(x) sin(clamp(x,0.,1.)*PI)

#define CrossSize floor(18.*sqrt(iResolution.x/650.))

float tuch(vec2 coord) {
    coord = coord * 2. + .25;
    vec4 rnd1 = vec4(0.5);
    vec4 rnd2 = vec4(0.5);
    float p = 0.;
    vec2 c = clamp(cos01(coord) * 1., -1., 1.);
    vec2 c2 = abs(clamp(sin01(coord * 2.6), -1., 1.)) * .5 + .5;
    p = max(p, (c.x * c.y * .5 + .5 + .4 * rnd2.x) * 1. * abs(c.x + .4 * (rnd2.x - .5)) * c2.x);
    p = max(p, (-c.x * c.y * .5 + .5 + .4 * rnd1.x) * 1. * abs(c.y + .4 * (rnd1.x - .5)) * c2.y);
    return p;
}

float kreuz(vec2 coord) {
    float stitchWidth = 1.;
    vec4 rnd = vec4(0.5);
    coord += abs(cos01(coord.x * 1.)) * abs(cos01(coord.y * 1.)) * .3 * (rnd.xy - .5 + .15 * vec2(-1, 1));
    vec2 pc = fract(coord * 2. + .5);
    pc += .0 * sin01(pc);
    float faser1 = .7 + .3 * abs(sin01((pc.x + pc.y) * 1.7 - (pc.x - pc.y) * .5));
    float faser2 = .7 + .3 * abs(sin01((pc.x - pc.y) * 1.7 + (pc.x + pc.y) * .5));
    return max(-.15 + humpW(pc.x + pc.y - 1., stitchWidth) * faser1 * 1.2, .15 + humpW(pc.x - pc.y, stitchWidth) * faser2 * 1.2) * (.5 + .5 * hump01(pc.x) * hump01(pc.y)) * 2.4;
}

float gauss(vec2 v) {
    return exp(-dot(v, v) * 25.);
}

float kreuzAO(vec2 coord) {
    //return 1.;
    vec4 rnd = vec4(0.5);
    coord += abs(cos01(coord.x * 1.)) * abs(cos01(coord.y * 1.)) * .3 * (rnd.xy - .5 + .15 * vec2(-1, 1));
    vec2 pc = fract(coord * 2. + .5);
    return 1. - .6 * (gauss(pc) + gauss(pc - vec2(1, 0)) + gauss(pc - vec2(1, 1)) + gauss(pc - vec2(0, 1)));
}

vec3 quant(vec3 c, ivec3 num) {
    vec3 fnum = vec3(num);
    return floor(c * (fnum - .0001)) / (fnum - 1.);
}

vec3 rgb2hsv(vec3 c) {
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 quantHSV(vec3 c, ivec3 num) {
    return hsv2rgb(quant(rgb2hsv(c) * vec3(1, 1.1, 1.1), num));
}

vec3 vid(vec2 coord) {
    return clamp(quantHSV(texture(iChannel0, coord / iResolution.xy).xyz, ivec3(48, 6, 6)), 0., 1.);
}

void frame(inout vec3 col, inout float isStitch, ivec2 coord, ivec2 s, int w) {
    if(abs(coord.y - s.y) > s.y - w) {
        isStitch = 0.;
        col = vec3(1);
    };
    if(abs(coord.x - s.x) > s.x - w) {
        isStitch = 0.;
        col = vec3(1);
    };
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec3 clothCol = vec3(1, .95, .8);
    vec2 coord = fragCoord;
    float crossSize = CrossSize;
    vec2 tc = coord / crossSize;
    float tuchPat = sqrt(tuch(tc));
    tuchPat = clamp(tuchPat, 0., 1.);
    vec2 vidCoord = floor(coord * 2. / crossSize + .5) / 2. * crossSize;
    #define FrameWidth 6
    vec2 vc;
    vc = (vidCoord - .5 * iResolution.xy) * iResolution.xy / (iResolution.xy - float(FrameWidth - 1) * crossSize) + .5 * iResolution.xy;
    vec3 vidCol = vid(vc);
    float kreuzPat = kreuz(tc);
    float pixGrad = (1. - .5 * length(fract(coord * 2. / crossSize + .5) - .5));

    float isStitch = 1.;

    // green screen
    isStitch = 1. - step(.5, dot(vidCol.xyz, vec3(-1, 2, -1)));

    vidCol = vidCol * .75 + .1;
    frame(vidCol, isStitch, ivec2(floor(vidCoord * 2. / crossSize)), ivec2(iResolution.xy / crossSize + .5), FrameWidth);

    float allPat = max(tuchPat, isStitch * kreuzPat);
    vec3 col = mix(vec3(1, .9, .8), vidCol * 1.5, isStitch * kreuzPat);
    col = vec3(1);
    fragColor.xyz = allPat * col;
    vec2 ds = (fragCoord.xy - iResolution.xy * .5) / iResolution.xy * 2.;
    fragColor.xyz = (allPat > tuchPat) ? vidCol * kreuzAO(tc) : clothCol;

    fragColor.w = allPat;
}
