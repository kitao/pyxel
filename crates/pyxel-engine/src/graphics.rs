use std::cmp::{max, min};
use std::collections::HashMap;
use std::mem::size_of;

use glow::HasContext;
use once_cell::sync::Lazy;

use crate::image::{Color, Image, Rgb8, SharedImage};
use crate::pyxel::Pyxel;
use crate::settings::{
    BACKGROUND_COLOR, CURSOR_DATA, CURSOR_HEIGHT, CURSOR_WIDTH, DEFAULT_COLORS, FONT_DATA,
    FONT_HEIGHT, FONT_WIDTH, NUM_FONT_ROWS,
};

pub(crate) static COLORS: Lazy<shared_type!(Vec<Rgb8>)> =
    Lazy::new(|| new_shared_type!(DEFAULT_COLORS.to_vec()));

pub(crate) static CURSOR_IMAGE: Lazy<SharedImage> = Lazy::new(|| {
    let image = Image::new(CURSOR_WIDTH, CURSOR_HEIGHT);
    image.lock().set(0, 0, &CURSOR_DATA);
    image
});

pub(crate) static FONT_IMAGE: Lazy<SharedImage> = Lazy::new(|| {
    let width = FONT_WIDTH * NUM_FONT_ROWS;
    let height = FONT_HEIGHT * ((FONT_DATA.len() as u32 + NUM_FONT_ROWS - 1) / NUM_FONT_ROWS);
    let image = Image::new(width, height);
    {
        let mut image = image.lock();
        for (fi, data) in FONT_DATA.iter().enumerate() {
            let row = fi as u32 / NUM_FONT_ROWS;
            let col = fi as u32 % NUM_FONT_ROWS;
            let mut data = *data;
            for yi in 0..FONT_HEIGHT {
                for xi in 0..FONT_WIDTH {
                    let x = FONT_WIDTH * col + xi;
                    let y = FONT_HEIGHT * row + yi;
                    let color = u8::from((data & 0x800000) != 0);
                    image.canvas.write_data(x as usize, y as usize, color);
                    data <<= 1;
                }
            }
        }
    }
    image
});

struct DisplayShader {
    shader_program: glow::Program,
    uniform_locations: HashMap<String, glow::NativeUniformLocation>,
}

pub struct Graphics {
    screen_texture: glow::NativeTexture,
    palette_texture: glow::NativeTexture,
    vertex_array: glow::VertexArray,
    plain_shader: DisplayShader,
}

impl Graphics {
    pub fn new() -> Self {
        unsafe {
            let gl = pyxel_platform::glow_context();
            let screen_texture = Self::create_screen_texture(gl);
            let palette_texture = Self::create_palette_texture(gl);
            let vertex_array = Self::create_vertex_array(gl);
            let plain_shader = Self::create_plain_shader(gl);
            Self {
                screen_texture,
                palette_texture,
                vertex_array,
                plain_shader,
            }
        }
    }

    unsafe fn create_vertex_array(gl: &mut glow::Context) -> glow::VertexArray {
        let vertex_array = gl.create_vertex_array().unwrap();
        let vertex_buffer = gl.create_buffer().unwrap();
        let element_array = gl.create_buffer().unwrap();
        let vertices: [f32; 16] = [
            -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
        let indices: [u32; 4] = [0, 2, 1, 3];
        gl.bind_vertex_array(Some(vertex_array));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            &vertices.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(element_array));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            &indices.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 4 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            glow::FLOAT,
            false,
            4 * size_of::<f32>() as i32,
            2 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(1);
        vertex_array
    }

    unsafe fn create_plain_shader(gl: &mut glow::Context) -> DisplayShader {
        // Vertex shader
        const VERTEX_SHADER_SOURCE: &str = r#"
            #ifdef GL_ES
            precision mediump float;
            #define MEDIUMP mediump
            #else
            #define MEDIUMP
            #endif

            attribute MEDIUMP vec2 a_position;
            attribute MEDIUMP vec2 a_texcoord;
            varying MEDIUMP vec2 v_texcoord;

            void main() {
                gl_Position = vec4(a_position, 0.0, 1.0);
                v_texcoord = a_texcoord;
            }
        "#;
        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vertex_shader, VERTEX_SHADER_SOURCE);
        gl.compile_shader(vertex_shader);

        // Fragment shader
        const FRAGMENT_SHADER_SOURCE: &str = r#"
            #ifdef GL_ES
            precision mediump float;
            #define MEDIUMP mediump
            #else
            #define MEDIUMP
            #endif

            varying MEDIUMP vec2 v_texcoord;
            uniform sampler2D u_screen_texture;
            uniform sampler2D u_palette_texture;
            uniform MEDIUMP uint u_num_colors;

            void main() {
                float index_color = texture2D(u_screen_texture, v_texcoord).r * 255.0;
                float palette_u = index_color / u_num_colors + 0.5;
                vec3 color = texture2D(u_palette_texture, vec2(palette_u, 0.5)).rgb;
                gl_FragColor = vec4(color, 1.0);
            }
        "#;
        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fragment_shader, FRAGMENT_SHADER_SOURCE);
        gl.compile_shader(fragment_shader);

        // Shader program
        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(shader_program, vertex_shader);
        gl.attach_shader(shader_program, fragment_shader);
        gl.link_program(shader_program);
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        // Uniform locations
        let mut uniform_locations = HashMap::new();
        let uniforms = ["u_screen_texture", "u_palette_texture", "u_num_colors"];
        for &uniform in &uniforms {
            let location = gl.get_uniform_location(shader_program, uniform).unwrap();
            uniform_locations.insert(String::from(uniform), location);
        }

        DisplayShader {
            shader_program,
            uniform_locations,
        }
    }

    unsafe fn create_screen_texture(gl: &mut glow::Context) -> glow::NativeTexture {
        let screen_texture = gl.create_texture().unwrap();
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D, Some(screen_texture));
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        screen_texture
    }

    unsafe fn create_palette_texture(gl: &mut glow::Context) -> glow::NativeTexture {
        let palette_texture = gl.create_texture().unwrap();
        gl.active_texture(glow::TEXTURE1);
        gl.bind_texture(glow::TEXTURE_2D, Some(palette_texture));
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        palette_texture
    }
}

impl Pyxel {
    pub fn image_no(&self, image: SharedImage) -> Option<u32> {
        for (i, builtin_image) in self.images.iter().enumerate() {
            if builtin_image.data_ptr() == image.data_ptr() {
                return Some(i as u32);
            }
        }
        None
    }

    pub fn clip(&self, x: f64, y: f64, width: f64, height: f64) {
        self.screen.lock().clip(x, y, width, height);
    }

    pub fn clip0(&self) {
        self.screen.lock().clip0();
    }

    pub fn camera(&self, x: f64, y: f64) {
        self.screen.lock().camera(x, y);
    }

    pub fn camera0(&self) {
        self.screen.lock().camera0();
    }

    pub fn pal(&self, src_color: Color, dst_color: Color) {
        self.screen.lock().pal(src_color, dst_color);
    }

    pub fn pal0(&self) {
        self.screen.lock().pal0();
    }

    pub fn cls(&self, color: Color) {
        self.screen.lock().cls(color);
    }

    pub fn pget(&self, x: f64, y: f64) -> Color {
        self.screen.lock().pget(x, y)
    }

    pub fn pset(&self, x: f64, y: f64, color: Color) {
        self.screen.lock().pset(x, y, color);
    }

    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
        self.screen.lock().line(x1, y1, x2, y2, color);
    }

    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().rect(x, y, width, height, color);
    }

    pub fn rectb(&self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().rectb(x, y, width, height, color);
    }

    pub fn circ(&self, x: f64, y: f64, radius: f64, color: Color) {
        self.screen.lock().circ(x, y, radius, color);
    }

    pub fn circb(&self, x: f64, y: f64, radius: f64, color: Color) {
        self.screen.lock().circb(x, y, radius, color);
    }

    pub fn elli(&self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().elli(x, y, width, height, color);
    }

    pub fn ellib(&self, x: f64, y: f64, width: f64, height: f64, color: Color) {
        self.screen.lock().ellib(x, y, width, height, color);
    }

    pub fn tri(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.screen.lock().tri(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn trib(&self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
        self.screen.lock().trib(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn fill(&self, x: f64, y: f64, color: Color) {
        self.screen.lock().fill(x, y, color);
    }

    pub fn blt(
        &self,
        x: f64,
        y: f64,
        image_no: u32,
        image_x: f64,
        image_y: f64,
        width: f64,
        height: f64,
        color_key: Option<Color>,
    ) {
        self.screen.lock().blt(
            x,
            y,
            self.images[image_no as usize].clone(),
            image_x,
            image_y,
            width,
            height,
            color_key,
        );
    }

    pub fn bltm(
        &self,
        x: f64,
        y: f64,
        tilemap_no: u32,
        tilemap_x: f64,
        tilemap_y: f64,
        width: f64,
        height: f64,
        color_key: Option<Color>,
    ) {
        self.screen.lock().bltm(
            x,
            y,
            self.tilemaps[tilemap_no as usize].clone(),
            tilemap_x,
            tilemap_y,
            width,
            height,
            color_key,
        );
    }

    pub fn text(&self, x: f64, y: f64, string: &str, color: Color) {
        self.screen.lock().text(x, y, string, color);
    }

    pub(crate) fn render_screen(&mut self) {
        unsafe {
            let gl = pyxel_platform::glow_context();
            self.clear_window(gl);
            self.bind_screen_texture(gl);
            self.bind_palette_texture(gl);
            self.use_plain_shader(gl);
            gl.bind_vertex_array(Some(self.graphics.vertex_array));
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            pyxel_platform::swap_window();
        }
    }

    unsafe fn clear_window(&self, gl: &mut glow::Context) {
        let r = ((BACKGROUND_COLOR >> 16) & 0xff) as f32 / 255.0;
        let g = ((BACKGROUND_COLOR >> 8) & 0xff) as f32 / 255.0;
        let b = (BACKGROUND_COLOR & 0xff) as f32 / 255.0;
        gl.clear_color(r, g, b, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }

    unsafe fn bind_screen_texture(&self, gl: &mut glow::Context) {
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.graphics.screen_texture));
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::R8 as i32,
            self.width as i32,
            self.height as i32,
            0,
            glow::RED,
            glow::UNSIGNED_BYTE,
            Some(&self.screen.lock().canvas.data),
        );
    }

    unsafe fn bind_palette_texture(&self, gl: &mut glow::Context) {
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.graphics.palette_texture));
        let colors = self.colors.lock();
        let mut pixels: Vec<u8> = Vec::with_capacity(colors.len() * 3);
        for rgb8 in &*colors {
            pixels.push(((rgb8 >> 16) & 0xff) as u8);
            pixels.push(((rgb8 >> 8) & 0xff) as u8);
            pixels.push((rgb8 & 0xff) as u8);
        }
        gl.tex_image_2d(
            glow::TEXTURE_1D,
            0,
            glow::RGB8 as i32,
            colors.len() as i32,
            1,
            0,
            glow::RGB,
            glow::UNSIGNED_BYTE,
            Some(&pixels),
        );
    }

    unsafe fn use_plain_shader(&self, gl: &mut glow::Context) {
        let (window_width, window_height) = pyxel_platform::window_size();
        let screen_scale = max(
            min(window_width / self.width, window_height / self.width),
            1,
        );
        let screen_width = self.width * screen_scale;
        let screen_height = self.height * screen_scale;
        let screen_x = (window_width as i32 - screen_width as i32) / 2;
        let screen_y = (window_height as i32 - screen_height as i32) / 2;
        gl.viewport(
            screen_x as i32,
            window_height as i32 - screen_y as i32,
            screen_width as i32,
            screen_height as i32,
        );
        let uniform_locations = &self.graphics.plain_shader.uniform_locations;
        gl.uniform_1_i32(uniform_locations.get("u_screen_texture"), 0);
        gl.uniform_1_i32(uniform_locations.get("u_palette_texture"), 1);
        gl.uniform_1_u32(
            uniform_locations.get("u_num_colors"),
            self.colors.lock().len() as u32,
        );
    }
}
