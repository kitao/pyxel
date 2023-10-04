vec2 curve(vec2 uv) {
    uv = (uv - 0.5) * 2.0;
    uv *= 1.1;
    uv.x *= 1.0 + pow((abs(uv.y) / 5.0), 2.0);
    uv.y *= 1.0 + pow((abs(uv.x) / 4.0), 2.0);
    uv = (uv / 2.0) + 0.5;
    uv = uv * 0.92 + 0.04;
    return uv;
}

void main() {
    vec2 screenSize, screenFragCoord, screenTexCoord;
    getScreenTexCoord(screenSize, screenFragCoord, screenTexCoord);
    vec2 uv = screenTexCoord;
    uv = curve(uv);
    if(!isInScreen(uv)) {
        fragColor = vec4(u_backgroundColor, 1.0);
        return;
    }

    vec3 col;
    col.r = getScreenColor(vec2(uv.x + 0.001, uv.y + 0.001)).x + 0.05;
    col.g = getScreenColor(vec2(uv.x + 0.000, uv.y - 0.002)).y + 0.05;
    col.b = getScreenColor(vec2(uv.x - 0.002, uv.y + 0.000)).z + 0.05;
    col.r += 0.08 * getScreenColor(0.75 * vec2(0.025, -0.027) + vec2(uv.x + 0.001, uv.y + 0.001)).x;
    col.g += 0.05 * getScreenColor(0.75 * vec2(-0.022, -0.02) + vec2(uv.x + 0.000, uv.y - 0.002)).y;
    col.b += 0.08 * getScreenColor(0.75 * vec2(-0.02, -0.018) + vec2(uv.x - 0.002, uv.y + 0.000)).z;
    col = clamp(col * 0.6 + 0.4 * col * col * 1.0, 0.0, 1.0);

    float vig = (0.0 + 1.0 * 16.0 * uv.x * uv.y * (1.0 - uv.x) * (1.0 - uv.y));
    col *= vec3(pow(vig, 0.3));
    col *= vec3(0.95, 1.05, 0.95);
    col *= 2.8;

    // TODO: iResolution.y
    float scans = clamp(0.35 + 0.35 * sin(uv.y * screenSize.y * 1.5), 0.0, 1.0);
    col = col * vec3(0.4 + 0.7 * pow(scans, 1.7));
    col *= 1.0 - 0.65 * vec3(clamp((mod(screenFragCoord.x, 2.0) - 1.0) * 2.0, 0.0, 1.0));

    fragColor = vec4(col, 1.0);
}
