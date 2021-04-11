#[
#
# Image class
#
int image_width_getter(void* self);
int image_height_getter(void* self);
int** image_data_getter(void* self);

int image_get(void* self, int x, int y);
void image_set1(void* self, int x, int y, int data);
void image_set(void* self,
               int x,
               int y,
               const char** data,
                     int data_length);
void image_load(void* self, int x, int y, const char* filename);
void image_copy(void* self, int x, int y, int img, int u, int v, int w, int h);
]#
