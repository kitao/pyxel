void main() {
    vec2 screenFragCoord, screenTexCoord;
    getScreenParams(screenFragCoord, screenTexCoord);
    if(isInScreen(screenTexCoord)) {
        fragColor = getScreenColor(screenTexCoord);
    } else {
        fragColor = vec4(u_backgroundColor, 1.0);
    }
}
