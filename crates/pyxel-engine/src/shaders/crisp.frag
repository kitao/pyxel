void main() {
    vec2 screenFragCoord, screenTexCoord;
    getScreenParams(screenFragCoord, screenTexCoord);
    if (isInScreen(screenTexCoord)) {
        fragColor = vec4(getScreenColor(screenTexCoord), 1.0);
    } else {
        fragColor = vec4(u_backgroundColor, 1.0);
    }
}
