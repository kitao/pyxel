out vec4 fragColor;

uniform vec2 u_windowSize;
uniform vec3 u_backgroundColor;
uniform sampler2D u_screenTexture;
uniform isampler2D u_colorsTexture;

vec2 getScreenResolution() {
    return textureSize(u_screenTexture, 0);
}

void getScreenParams(out vec2 screenSize, out vec2 screenFragCoord, out vec2 screenTexCoord) {
    vec2 screenResolution = getScreenResolution();
    vec2 screenScaleVec = max(floor(u_windowSize / screenResolution), vec2(1.0));
    float screenScale = min(screenScaleVec.x, screenScaleVec.y);
    screenSize = screenResolution * screenScale;
    vec2 screenPos = vec2(floor((u_windowSize.x - screenSize.x) / 2.0), ceil((u_windowSize.y - screenSize.y) / 2.0));
    screenFragCoord = gl_FragCoord.xy - screenPos;
    screenTexCoord = vec2(screenFragCoord.x, (screenSize.y - screenFragCoord.y)) / screenSize;
}

bool isInScreen(vec2 screenTexCoord) {
    return all(greaterThanEqual(screenTexCoord, vec2(0.0))) && all(lessThanEqual(screenTexCoord, vec2(1.0)));
}

vec4 getScreenColor(vec2 screenTexCoord) {
    float indexColor = texture(u_screenTexture, screenTexCoord).r * 255.0;
    vec2 colorsTexCoord = vec2((indexColor + 0.5) / float(textureSize(u_colorsTexture, 0).x), 0.5);
    uint rgb = uint(texture(u_colorsTexture, colorsTexCoord).r);
    uint r = (rgb >> 16) & 0xffu;
    uint g = (rgb >> 8) & 0xffu;
    uint b = rgb & 0xffu;
    return vec4(vec3(r, g, b) / 255.0, 1.0);
}
