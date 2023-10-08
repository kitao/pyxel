out vec4 fragColor;

uniform vec2 u_screenPos;
uniform vec2 u_screenSize;
uniform float u_screenScale;
uniform vec3 u_backgroundColor;
uniform sampler2D u_screenTexture;
uniform isampler2D u_colorsTexture;

void getScreenParams(out vec2 screenFragCoord, out vec2 screenTexCoord) {
    screenFragCoord = gl_FragCoord.xy - u_screenPos;
    screenTexCoord = screenFragCoord / u_screenSize;
    screenTexCoord.y = 1.0 - screenTexCoord.y;
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
