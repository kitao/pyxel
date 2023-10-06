void main() {
    vec2 screenSize, screenFragCoord, screenTexCoord;
    getScreenParams(screenSize, screenFragCoord, screenTexCoord);
    if(isInScreen(screenTexCoord)) {
        fragColor = getScreenColor(screenTexCoord);
    } else {
        fragColor = vec4(u_backgroundColor, 1.0);
    }
}
