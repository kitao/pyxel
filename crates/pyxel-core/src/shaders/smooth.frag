/*
   Based on xBR shaders from:
   https://github.com/libretro/common-shaders/tree/master/xbr/shaders

   Pyxel-specific changes are commented in place.
*/

/*
   Hyllian's xBR-vertex code and texel mapping

   Copyright (C) 2011/2016 Hyllian - sergiogdb@gmail.com

   Permission is hereby granted, free of charge, to any person obtaining a copy
   of this software and associated documentation files (the "Software"), to deal
   in the Software without restriction, including without limitation the rights
   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
   copies of the Software, and to permit persons to whom the Software is
   furnished to do so, subject to the following conditions:

   The above copyright notice and this permission notice shall be included in
   all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
   THE SOFTWARE.

*/

// Map the original shader outputs and sizes to Pyxel's uniforms.
#define FragColor gl_FragColor
#define OutputSize u_screenSize
#define TextureSize (u_screenSize / u_screenScale)
#define vTexCoord screenTexCoord

#define SourceSize vec4(TextureSize, 1.0 / TextureSize) // Original xBR source-size layout.
#define OutSize vec4(OutputSize, 1.0 / OutputSize)

#define BLEND_NONE 0
#define BLEND_NORMAL 1
#define BLEND_DOMINANT 2
#define LUMINANCE_WEIGHT 1.0
#define EQUAL_COLOR_TOLERANCE (30.0 / 255.0)
#define STEEP_DIRECTION_THRESHOLD 2.2
#define DOMINANT_DIRECTION_THRESHOLD 3.6

float distYCbCr(vec3 pixA, vec3 pixB) {
    const vec3 w = vec3(0.2627, 0.6780, 0.0593);
    const float scaleB = 0.5 / (1.0 - w.b);
    const float scaleR = 0.5 / (1.0 - w.r);
    vec3 diff = pixA - pixB;
    float Y = dot(diff.rgb, w);
    float Cb = scaleB * (diff.b - Y);
    float Cr = scaleR * (diff.r - Y);

    return sqrt(((LUMINANCE_WEIGHT * Y) * (LUMINANCE_WEIGHT * Y)) + (Cb * Cb) + (Cr * Cr));
}

bool isPixEqual(const vec3 pixA, const vec3 pixB) {
    return (distYCbCr(pixA, pixB) < EQUAL_COLOR_TOLERANCE);
}

float getLeftRatio(vec2 center, vec2 origin, vec2 direction, vec2 scale) {
    vec2 P0 = center - origin;
    vec2 proj = direction * (dot(P0, direction) / dot(direction, direction));
    vec2 distv = P0 - proj;
    vec2 orth = vec2(-direction.y, direction.x);
    float side = sign(dot(P0, orth));
    float v = side * length(distv * scale);

    return smoothstep(-sqrt(2.0) / 2.0, sqrt(2.0) / 2.0, v);
}

#define eq(a, b) (a == b)
#define neq(a, b) (a != b)

// Sample through Pyxel's indexed-color screen texture.
#define P(x,y) getScreenColor(coord + SourceSize.zw * vec2(x, y))

void main() {
    // Resolve Pyxel screen coordinates before the xBR pass.
    vec2 screenFragCoord, screenTexCoord;
    getScreenParams(screenFragCoord, screenTexCoord);
    if (!isInScreen(screenTexCoord)) {
        FragColor = vec4(u_backgroundColor, 1.0);
        return;
    }

    // Input Pixel Mapping:  -|x|x|x|-
    //                       x|A|B|C|x
    //                       x|D|E|F|x
    //                       x|G|H|I|x
    //                       -|x|x|x|-

    vec2 scale = OutputSize.xy * SourceSize.zw;
    vec2 pos = fract(vTexCoord * SourceSize.xy) - vec2(0.5, 0.5);
    vec2 coord = vTexCoord - pos * SourceSize.zw;

    vec3 A = P(-1., -1.);
    vec3 B = P(0., -1.);
    vec3 C = P(1., -1.);
    vec3 D = P(-1., 0.);
    vec3 E = P(0., 0.);
    vec3 F = P(1., 0.);
    vec3 G = P(-1., 1.);
    vec3 H = P(0., 1.);
    vec3 I = P(1., 1.);

    // blendResult Mapping: x|y|
    //                      w|z|
    ivec4 blendResult = ivec4(BLEND_NONE, BLEND_NONE, BLEND_NONE, BLEND_NONE);

    // Preprocess corners
    // Pixel Tap Mapping: -|-|-|-|-
    //                    -|-|B|C|-
    //                    -|D|E|F|x
    //                    -|G|H|I|x
    //                    -|-|x|x|-
    if (!((eq(E, F) && eq(H, I)) || (eq(E, H) && eq(F, I)))) {
        float dist_H_F = distYCbCr(G, E) + distYCbCr(E, C) + distYCbCr(P(0, 2), I) + distYCbCr(I, P(2., 0.)) + (4.0 * distYCbCr(H, F));
        float dist_E_I = distYCbCr(D, H) + distYCbCr(H, P(1, 2)) + distYCbCr(B, F) + distYCbCr(F, P(2., 1.)) + (4.0 * distYCbCr(E, I));
        bool dominantGradient = (DOMINANT_DIRECTION_THRESHOLD * dist_H_F) < dist_E_I;
        blendResult.z = ((dist_H_F < dist_E_I) && neq(E, F) && neq(E, H)) ? ((dominantGradient) ? BLEND_DOMINANT : BLEND_NORMAL) : BLEND_NONE;
    }

    // Pixel Tap Mapping: -|-|-|-|-
    //                    -|A|B|-|-
    //                    x|D|E|F|-
    //                    x|G|H|I|-
    //                    -|x|x|-|-
    if (!((eq(D, E) && eq(G, H)) || (eq(D, G) && eq(E, H)))) {
        float dist_G_E = distYCbCr(P(-2., 1.), D) + distYCbCr(D, B) + distYCbCr(P(-1., 2.), H) + distYCbCr(H, F) + (4.0 * distYCbCr(G, E));
        float dist_D_H = distYCbCr(P(-2., 0.), G) + distYCbCr(G, P(0., 2.)) + distYCbCr(A, E) + distYCbCr(E, I) + (4.0 * distYCbCr(D, H));
        bool dominantGradient = (DOMINANT_DIRECTION_THRESHOLD * dist_D_H) < dist_G_E;
        blendResult.w = ((dist_G_E > dist_D_H) && neq(E, D) && neq(E, H)) ? ((dominantGradient) ? BLEND_DOMINANT : BLEND_NORMAL) : BLEND_NONE;
    }

    // Pixel Tap Mapping: -|-|x|x|-
    //                    -|A|B|C|x
    //                    -|D|E|F|x
    //                    -|-|H|I|-
    //                    -|-|-|-|-
    if (!((eq(B, C) && eq(E, F)) || (eq(B, E) && eq(C, F)))) {
        float dist_E_C = distYCbCr(D, B) + distYCbCr(B, P(1., -2.)) + distYCbCr(H, F) + distYCbCr(F, P(2., -1.)) + (4.0 * distYCbCr(E, C));
        float dist_B_F = distYCbCr(A, E) + distYCbCr(E, I) + distYCbCr(P(0., -2.), C) + distYCbCr(C, P(2., 0.)) + (4.0 * distYCbCr(B, F));
        bool dominantGradient = (DOMINANT_DIRECTION_THRESHOLD * dist_B_F) < dist_E_C;
        blendResult.y = ((dist_E_C > dist_B_F) && neq(E, B) && neq(E, F)) ? ((dominantGradient) ? BLEND_DOMINANT : BLEND_NORMAL) : BLEND_NONE;
    }

    // Pixel Tap Mapping: -|x|x|-|-
    //                    x|A|B|C|-
    //                    x|D|E|F|-
    //                    -|G|H|-|-
    //                    -|-|-|-|-
    if (!((eq(A, B) && eq(D, E)) || (eq(A, D) && eq(B, E)))) {
        float dist_D_B = distYCbCr(P(-2., 0.), A) + distYCbCr(A, P(0., -2.)) + distYCbCr(G, E) + distYCbCr(E, C) + (4.0 * distYCbCr(D, B));
        float dist_A_E = distYCbCr(P(-2., -1.), D) + distYCbCr(D, H) + distYCbCr(P(-1., -2.), B) + distYCbCr(B, F) + (4.0 * distYCbCr(A, E));
        bool dominantGradient = (DOMINANT_DIRECTION_THRESHOLD * dist_D_B) < dist_A_E;
        blendResult.x = ((dist_D_B < dist_A_E) && neq(E, D) && neq(E, B)) ? ((dominantGradient) ? BLEND_DOMINANT : BLEND_NORMAL) : BLEND_NONE;
    }

    vec3 res = E;

    // Pixel Tap Mapping: -|-|-|-|-
    //                    -|-|B|C|-
    //                    -|D|E|F|x
    //                    -|G|H|I|x
    //                    -|-|x|x|-
    if (blendResult.z != BLEND_NONE) {
        float dist_F_G = distYCbCr(F, G);
        float dist_H_C = distYCbCr(H, C);
        bool doLineBlend = (blendResult.z == BLEND_DOMINANT ||
            !((blendResult.y != BLEND_NONE && !isPixEqual(E, G)) || (blendResult.w != BLEND_NONE && !isPixEqual(E, C)) ||
            (isPixEqual(G, H) && isPixEqual(H, I) && isPixEqual(I, F) && isPixEqual(F, C) && !isPixEqual(E, I))));

        vec2 origin = vec2(0.0, 1.0 / sqrt(2.0));
        vec2 direction = vec2(1.0, -1.0);
        if (doLineBlend) {
            bool haveShallowLine = (STEEP_DIRECTION_THRESHOLD * dist_F_G <= dist_H_C) && neq(E, G) && neq(D, G);
            bool haveSteepLine = (STEEP_DIRECTION_THRESHOLD * dist_H_C <= dist_F_G) && neq(E, C) && neq(B, C);
            origin = haveShallowLine ? vec2(0.0, 0.25) : vec2(0.0, 0.5);
            direction.x += haveShallowLine ? 1.0 : 0.0;
            direction.y -= haveSteepLine ? 1.0 : 0.0;
        }

        vec3 blendPix = mix(H, F, step(distYCbCr(E, F), distYCbCr(E, H)));
        res = mix(res, blendPix, getLeftRatio(pos, origin, direction, scale));
    }

    // Pixel Tap Mapping: -|-|-|-|-
    //                    -|A|B|-|-
    //                    x|D|E|F|-
    //                    x|G|H|I|-
    //                    -|x|x|-|-
    if (blendResult.w != BLEND_NONE) {
        float dist_H_A = distYCbCr(H, A);
        float dist_D_I = distYCbCr(D, I);
        bool doLineBlend = (blendResult.w == BLEND_DOMINANT ||
            !((blendResult.z != BLEND_NONE && !isPixEqual(E, A)) || (blendResult.x != BLEND_NONE && !isPixEqual(E, I)) ||
            (isPixEqual(A, D) && isPixEqual(D, G) && isPixEqual(G, H) && isPixEqual(H, I) && !isPixEqual(E, G))));

        vec2 origin = vec2(-1.0 / sqrt(2.0), 0.0);
        vec2 direction = vec2(1.0, 1.0);
        if (doLineBlend) {
            bool haveShallowLine = (STEEP_DIRECTION_THRESHOLD * dist_H_A <= dist_D_I) && neq(E, A) && neq(B, A);
            bool haveSteepLine = (STEEP_DIRECTION_THRESHOLD * dist_D_I <= dist_H_A) && neq(E, I) && neq(F, I);
            origin = haveShallowLine ? vec2(-0.25, 0.0) : vec2(-0.5, 0.0);
            direction.y += haveShallowLine ? 1.0 : 0.0;
            direction.x += haveSteepLine ? 1.0 : 0.0;
        }
        vec3 blendPix = mix(H, D, step(distYCbCr(E, D), distYCbCr(E, H)));
        res = mix(res, blendPix, getLeftRatio(pos, origin, direction, scale));
    }

    // Pixel Tap Mapping: -|-|x|x|-
    //                    -|A|B|C|x
    //                    -|D|E|F|x
    //                    -|-|H|I|-
    //                    -|-|-|-|-
    if (blendResult.y != BLEND_NONE) {
        float dist_B_I = distYCbCr(B, I);
        float dist_F_A = distYCbCr(F, A);
        bool doLineBlend = (blendResult.y == BLEND_DOMINANT ||
            !((blendResult.x != BLEND_NONE && !isPixEqual(E, I)) || (blendResult.z != BLEND_NONE && !isPixEqual(E, A)) ||
            (isPixEqual(I, F) && isPixEqual(F, C) && isPixEqual(C, B) && isPixEqual(B, A) && !isPixEqual(E, C))));

        vec2 origin = vec2(1.0 / sqrt(2.0), 0.0);
        vec2 direction = vec2(-1.0, -1.0);
        if (doLineBlend) {
            bool haveShallowLine = (STEEP_DIRECTION_THRESHOLD * dist_B_I <= dist_F_A) && neq(E, I) && neq(H, I);
            bool haveSteepLine = (STEEP_DIRECTION_THRESHOLD * dist_F_A <= dist_B_I) && neq(E, A) && neq(D, A);
            origin = haveShallowLine ? vec2(0.25, 0.0) : vec2(0.5, 0.0);
            direction.y -= haveShallowLine ? 1.0 : 0.0;
            direction.x -= haveSteepLine ? 1.0 : 0.0;
        }

        vec3 blendPix = mix(F, B, step(distYCbCr(E, B), distYCbCr(E, F)));
        res = mix(res, blendPix, getLeftRatio(pos, origin, direction, scale));
    }

    // Pixel Tap Mapping: -|x|x|-|-
    //                    x|A|B|C|-
    //                    x|D|E|F|-
    //                    -|G|H|-|-
    //                    -|-|-|-|-
    if (blendResult.x != BLEND_NONE) {
        float dist_D_C = distYCbCr(D, C);
        float dist_B_G = distYCbCr(B, G);
        bool doLineBlend = (blendResult.x == BLEND_DOMINANT ||
            !((blendResult.w != BLEND_NONE && !isPixEqual(E, C)) || (blendResult.y != BLEND_NONE && !isPixEqual(E, G)) ||
            (isPixEqual(C, B) && isPixEqual(B, A) && isPixEqual(A, D) && isPixEqual(D, G) && !isPixEqual(E, A))));

        vec2 origin = vec2(0.0, -1.0 / sqrt(2.0));
        vec2 direction = vec2(-1.0, 1.0);
        if (doLineBlend) {
            bool haveShallowLine = (STEEP_DIRECTION_THRESHOLD * dist_D_C <= dist_B_G) && neq(E, C) && neq(F, C);
            bool haveSteepLine = (STEEP_DIRECTION_THRESHOLD * dist_B_G <= dist_D_C) && neq(E, G) && neq(H, G);
            origin = haveShallowLine ? vec2(0.0, -0.25) : vec2(0.0, -0.5);
            direction.x -= haveShallowLine ? 1.0 : 0.0;
            direction.y += haveSteepLine ? 1.0 : 0.0;
        }

        vec3 blendPix = mix(D, B, step(distYCbCr(E, B), distYCbCr(E, D)));
        res = mix(res, blendPix, getLeftRatio(pos, origin, direction, scale));
    }

    FragColor = vec4(res, 1.0);
}
