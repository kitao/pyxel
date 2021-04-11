#
# Graphics
#
void* image(int img, int system);
void* tilemap(int tm);
void clip0();
void clip(int x, int y, int w, int h);
void pal0();
void pal(int col1, int col2);
void cls(int col);
int pget(int x, int y);
void pset(int x, int y, int col);
void line(int x1, int y1, int x2, int y2, int col);
void rect(int x, int y, int w, int h, int col);
void rectb(int x, int y, int w, int h, int col);
void circ(int x, int y, int r, int col);
void circb(int x, int y, int r, int col);
void tri(int x1, int y1, int x2, int y2, int x3, int y3, int col);
void trib(int x1, int y1, int x2, int y2, int x3, int y3, int col);
void blt(int x, int y, int img, int u, int v, int w, int h, int colkey);
void bltm(int x, int y, int tm, int u, int v, int w, int h, int colkey);
void text(int x, int y, const char* s, int col);
