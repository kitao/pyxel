void main() {
    vec2 screenFragCoord, screenTexCoord;
    getScreenParams(screenFragCoord, screenTexCoord);
    if (!isInScreen(screenTexCoord)) {
        fragColor = vec4(u_backgroundColor, 1.0);
        return;
    }
    /*
        A B C
        D E F
        G H I
    */
    vec2 pixelW = vec2(u_screenScale, 0.0) / u_screenSize;
    vec2 pixelH = vec2(0.0, u_screenScale) / u_screenSize;
    vec3 A = getScreenColor(screenTexCoord - pixelW + pixelH);
    vec3 B = getScreenColor(screenTexCoord + pixelH);
    vec3 C = getScreenColor(screenTexCoord + pixelW + pixelH);
    vec3 D = getScreenColor(screenTexCoord - pixelW);
    vec3 E = getScreenColor(screenTexCoord);
    vec3 F = getScreenColor(screenTexCoord + pixelW);
    vec3 G = getScreenColor(screenTexCoord - pixelW - pixelH);
    vec3 H = getScreenColor(screenTexCoord - pixelH);
    vec3 I = getScreenColor(screenTexCoord + pixelW - pixelH);
    /*
        E0 E1 E2
        E3 E4 E5
        E6 E7 E8
    */
    vec3 color = E;
    vec2 offset = fract(screenTexCoord * u_screenSize / u_screenScale) * 3.0;
    if (D == B && B != F && D != H) {
        if (offset.x < 1.0 && offset.y >= 2.0) { // E0
            color = D;
        } else if (offset.x >= 1.0 && offset.x < 2.0 && offset.y >= 2.0 && E != C) { // E1
            color = B;
        } else if (offset.x < 1.0 && offset.y >= 1.0 && offset.y < 2.0 && E != G) { // E3
            color = D;
        }
    }
    if (B == F && B != D && F != H) {
        if (offset.x >= 2.0 && offset.y >= 2.0) { // E2
            color = F;
        } else if (offset.x >= 1.0 && offset.x < 2.0 && offset.y >= 2.0 && E != A) { // E1
            color = B;
        } else if (offset.x >= 2.0 && offset.y >= 1.0 && offset.y < 2.0 && E != I) { // E5
            color = F;
        }
    }
    if (D == H && D != B && H != F) {
        if (offset.x < 1.0 && offset.y < 1.0) { // E6
            color = D;
        } else if (offset.x < 1.0 && offset.y >= 1.0 && offset.y < 2.0 && E != A) { // E3
            color = D;
        } else if (offset.x >= 1.0 && offset.x < 2.0 && offset.y < 1.0 && E != I) { // E7
            color = H;
        }
    }
    if (H == F && D != H && B != F) {
        if (offset.x >= 2.0 && offset.y < 1.0) { // E8
            color = F;
        } else if (offset.x >= 2.0 && offset.y >= 1.0 && offset.y < 2.0 && E != C) { // E5
            color = F;
        } else if (offset.x >= 1.0 && offset.x < 2.0 && offset.y < 1.0 && E != G) { // E7
            color = H;
        }
    }
    fragColor = vec4(color, 1.0);
}
