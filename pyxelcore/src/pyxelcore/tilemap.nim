#
# Tilemap class
#
int tilemap_width_getter(void* self);
int tilemap_height_getter(void* self);
int** tilemap_data_getter(void* self);
int tilemap_refimg_getter(void* self);
void tilemap_refimg_setter(void* self, int refimg);

int tilemap_get(void* self, int x, int y);
void tilemap_set1(void* self, int x, int y, int data);
void tilemap_set(void* self,
                 int x,
                 int y,
                 const char** data,
                       int data_length);
void tilemap_copy(void* self, int x, int y, int tm, int u, int v, int w, int h);
