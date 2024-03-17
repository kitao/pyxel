vec2 warpScreen(vec2 screenTexCoord) {
    screenTexCoord = (screenTexCoord - 0.5) * 2.0;
    screenTexCoord *= 1.1;
    screenTexCoord.x *= 1.0 + pow((abs(screenTexCoord.y) / 5.0), 2.0);
    screenTexCoord.y *= 1.0 + pow((abs(screenTexCoord.x) / 4.0), 2.0);
    screenTexCoord = (screenTexCoord / 2.0) + 0.5;
    screenTexCoord = screenTexCoord * 0.92 + 0.04;
    return screenTexCoord;
}

vec3 getBleedingColor(vec2 screenTexCoord) {
    vec3 color;
    color.r = getScreenColor(vec2(screenTexCoord.x + 0.001, screenTexCoord.y + 0.001)).x + 0.05;
    color.g = getScreenColor(vec2(screenTexCoord.x + 0.000, screenTexCoord.y - 0.002)).y + 0.05;
    color.b = getScreenColor(vec2(screenTexCoord.x - 0.002, screenTexCoord.y + 0.000)).z + 0.05;
    color.r += 0.08 * getScreenColor(0.75 * vec2(0.025, -0.027) + vec2(screenTexCoord.x + 0.001, screenTexCoord.y + 0.001)).x;
    color.g += 0.05 * getScreenColor(0.75 * vec2(-0.022, -0.02) + vec2(screenTexCoord.x + 0.000, screenTexCoord.y - 0.002)).y;
    color.b += 0.08 * getScreenColor(0.75 * vec2(-0.02, -0.018) + vec2(screenTexCoord.x - 0.002, screenTexCoord.y + 0.000)).z;
    color = clamp(color * 0.6 + 0.4 * color * color * 1.0, 0.0, 1.0);
    return color;
}

vec3 getVignettFactor(vec2 screenTexCoord) {
    float vignette = (0.0 + 1.0 * 16.0 * screenTexCoord.x * screenTexCoord.y * (1.0 - screenTexCoord.x) * (1.0 - screenTexCoord.y));
    vec3 color = vec3(pow(vignette, 0.3));
    color *= vec3(0.95, 1.05, 0.95);
    color *= 2.8;
    return color;
}

float getScanlineFactor(vec2 screenFragCoord, vec2 screenTexCoord) {
    float scans = clamp(0.35 + 0.35 * sin(screenTexCoord.y * u_screenSize.y * 1.5), 0.0, 1.0);
    float color = 0.4 + 0.7 * pow(scans, 1.7);
    color *= 1.0 - 0.65 * clamp((mod(screenFragCoord.x, 2.0) - 1.0) * 2.0, 0.0, 1.0);
    return color;
}

void main() {
    vec2 screenFragCoord, screenTexCoord;
    getScreenParams(screenFragCoord, screenTexCoord);
    screenTexCoord = warpScreen(screenTexCoord);
    if (isInScreen(screenTexCoord)) {
        vec3 color = getBleedingColor(screenTexCoord);
        color *= getVignettFactor(screenTexCoord);
        color *= getScanlineFactor(screenFragCoord, screenTexCoord);
        gl_FragColor = vec4(color, 1.0);
    } else {
        gl_FragColor = vec4(u_backgroundColor, 1.0);
    }
}
