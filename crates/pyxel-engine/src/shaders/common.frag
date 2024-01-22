uniform vec2 u_screenPos;
uniform vec2 u_screenSize;
uniform float u_screenScale;
uniform int u_numColors;
uniform vec3 u_backgroundColor;
uniform sampler2D u_screenTexture;
uniform sampler2D u_colorsTexture;

void getScreenParams(out vec2 screenFragCoord, out vec2 screenTexCoord) {
    screenFragCoord = gl_FragCoord.xy - u_screenPos;
    screenTexCoord = screenFragCoord / u_screenSize;
    screenTexCoord.y = 1.0 - screenTexCoord.y;
}

bool isInScreen(vec2 screenTexCoord) {
    return all(greaterThanEqual(screenTexCoord, vec2(0.0))) && all(lessThanEqual(screenTexCoord, vec2(1.0)));
}

vec3 getScreenColor(vec2 screenTexCoord) {
    float indexColor = texture2D(u_screenTexture, screenTexCoord).r * 255.0;
    vec2 colorsTexCoord = vec2((indexColor + 0.5) / float(u_numColors), 0.5);
    return texture2D(u_colorsTexture, colorsTexCoord).rgb;
}
