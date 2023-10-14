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
    vec2 offset = fract(screenTexCoord * u_screenSize / u_screenScale) * 3.0;
    vec3 color = E;
    if (B != H && D != F) {
        if (offset.y >= 2.0) {
            if (offset.x < 1.0) { // E0
                color = (D == B) ? D : E;
            } else if (offset.x >= 2.0) { // E2
                color = (B == F) ? F : E;
            } else { // E1
                color = (D == B && E != C || B == F && E != A) ? B : E;
            }
        } else if (offset.y < 1.0) {
            if (offset.x < 1.0) { // E6
                color = (D == H) ? D : E;
            } else if (offset.x >= 2.0) { // E8
                color = (H == F) ? F : E;
            } else { // E7
                color = (D == H && E != I || H == F && E != G) ? H : E;
            }
        } else {
            if (offset.x < 1.0) { // E3
                color = (D == B && E != G || D == H && E != A) ? D : E;
            } else if (offset.x >= 2.0) { // E5
                color = (B == F && E != I || H == F && E != C) ? F : E;
            }
        }
    }
    fragColor = vec4(color, 1.0);
}
