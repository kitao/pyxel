void main() {
    vec2 screenFragCoord, screenTexCoord;
    getScreenParams(screenFragCoord, screenTexCoord);
    if (isInScreen(screenTexCoord)) {
        gl_FragColor = vec4(getScreenColor(screenTexCoord), 1.0);
    } else {
        gl_FragColor = vec4(u_backgroundColor, 1.0);
    }
}
