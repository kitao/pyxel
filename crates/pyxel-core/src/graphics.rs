use std::collections::HashMap;
use std::mem::size_of;

use glow::{HasContext, PixelUnpackData};

use crate::font::Font;
use crate::image::Color;
use crate::platform;
use crate::platform::GLProfile;
use crate::pyxel::{self, Pyxel};
use crate::settings::{BACKGROUND_COLOR, MAX_COLORS, NUM_SCREEN_TYPES};

#[cfg(target_os = "macos")]
const GL_VERSION: &str = include_str!("shaders/gles_version.glsl");
#[cfg(not(target_os = "macos"))]
const GL_VERSION: &str = include_str!("shaders/gl_version.glsl");

const GLES_VERSION: &str = include_str!("shaders/gles_version.glsl");
const COMMON_VERT: &str = include_str!("shaders/common.vert");
const COMMON_FRAG: &str = include_str!("shaders/common.frag");
const SCREEN_FRAGS: [&str; NUM_SCREEN_TYPES as usize] = [
    include_str!("shaders/crisp.frag"),
    include_str!("shaders/smooth.frag"),
    include_str!("shaders/retro.frag"),
];

pub struct ScreenShader {
    shader_program: glow::Program,
    uniform_locations: HashMap<String, glow::UniformLocation>,
    vertex_array: glow::VertexArray,
}

pub struct Graphics {
    screen_shaders: Vec<ScreenShader>,
    screen_texture: glow::NativeTexture,
    colors_texture: glow::NativeTexture,
}

impl Graphics {
    pub fn new() -> Self {
        unsafe {
            let gl = platform::gl_context();

            if platform::gl_profile() != GLProfile::Gles {
                gl.disable(glow::FRAMEBUFFER_SRGB);
            }
            gl.disable(glow::BLEND);

            let screen_shaders = Self::create_screen_shaders(gl);
            let screen_texture = Self::create_screen_texture(gl);
            let colors_texture = Self::create_colors_texture(gl);

            Self {
                screen_shaders,
                screen_texture,
                colors_texture,
            }
        }
    }

    unsafe fn create_screen_shaders(gl: &mut glow::Context) -> Vec<ScreenShader> {
        let glsl_version = if platform::gl_profile() == GLProfile::Gles {
            GLES_VERSION
        } else {
            GL_VERSION
        };

        let mut screen_shaders = Vec::new();
        for &screen_frag in &SCREEN_FRAGS {
            // Vertex shader
            let vertex_shader = gl
                .create_shader(glow::VERTEX_SHADER)
                .expect("Failed to create OpenGL vertex shader");
            gl.shader_source(vertex_shader, &format!("{glsl_version}{COMMON_VERT}"));
            gl.compile_shader(vertex_shader);
            assert!(
                gl.get_shader_compile_status(vertex_shader),
                "\n[vertex shader]\n{}",
                gl.get_shader_info_log(vertex_shader)
            );

            // Fragment shader
            let fragment_shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .expect("Failed to create OpenGL fragment shader");
            gl.shader_source(
                fragment_shader,
                &format!("{glsl_version}{COMMON_FRAG}{screen_frag}"),
            );
            gl.compile_shader(fragment_shader);
            assert!(
                gl.get_shader_compile_status(fragment_shader),
                "\n[fragment shader]\n{}",
                gl.get_shader_info_log(fragment_shader)
            );

            // Shader program
            let shader_program = gl
                .create_program()
                .expect("Failed to create OpenGL shader program");
            gl.attach_shader(shader_program, vertex_shader);
            gl.attach_shader(shader_program, fragment_shader);
            gl.link_program(shader_program);
            assert!(
                gl.get_program_link_status(shader_program),
                "{}",
                gl.get_program_info_log(shader_program)
            );
            gl.detach_shader(shader_program, vertex_shader);
            gl.delete_shader(vertex_shader);
            gl.detach_shader(shader_program, fragment_shader);
            gl.delete_shader(fragment_shader);

            // Uniform locations
            let mut uniform_locations: HashMap<String, glow::UniformLocation> = HashMap::new();
            let uniform_names = [
                "u_screenPos",
                "u_screenSize",
                "u_screenScale",
                "u_numColors",
                "u_backgroundColor",
                "u_screenTexture",
                "u_colorsTexture",
            ];

            for &uniform_name in &uniform_names {
                if let Some(location) = gl.get_uniform_location(shader_program, uniform_name) {
                    uniform_locations.insert(uniform_name.to_string(), location);
                }
            }

            // Vertex array
            let vertices: [f32; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];
            let vertex_array = gl
                .create_vertex_array()
                .expect("Failed to create OpenGL vertex array");
            let vertex_buffer = gl.create_buffer().expect("Failed to create OpenGL buffer");

            gl.bind_vertex_array(Some(vertex_array));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                vertices.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            let position = gl
                .get_attrib_location(shader_program, "position")
                .expect("Failed to find OpenGL attribute 'position'");
            gl.vertex_attrib_pointer_f32(
                position,
                2,
                glow::FLOAT,
                false,
                2 * size_of::<f32>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(position);

            // Add screen shader
            screen_shaders.push(ScreenShader {
                shader_program,
                uniform_locations,
                vertex_array,
            });
        }

        screen_shaders
    }

    unsafe fn create_screen_texture(gl: &mut glow::Context) -> glow::NativeTexture {
        let screen_texture = gl
            .create_texture()
            .expect("Failed to create OpenGL screen texture");
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

    unsafe fn create_colors_texture(gl: &mut glow::Context) -> glow::NativeTexture {
        let colors_texture = gl
            .create_texture()
            .expect("Failed to create OpenGL colors texture");
        gl.active_texture(glow::TEXTURE1);
        gl.bind_texture(glow::TEXTURE_2D, Some(colors_texture));

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

        colors_texture
    }
}

impl Pyxel {
    pub fn set_clip_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        pyxel::screen().set_clip_rect(x, y, width, height);
    }

    pub fn reset_clip_rect(&self) {
        pyxel::screen().reset_clip_rect();
    }

    pub fn set_draw_offset(&self, x: f32, y: f32) {
        pyxel::screen().set_draw_offset(x, y);
    }

    pub fn reset_draw_offset(&self) {
        pyxel::screen().reset_draw_offset();
    }

    pub fn map_color(&self, src_color: Color, dst_color: Color) {
        pyxel::screen().map_color(src_color, dst_color);
    }

    pub fn reset_color_map(&self) {
        pyxel::screen().reset_color_map();
    }

    pub fn set_dithering(&self, alpha: f32) {
        pyxel::screen().set_dithering(alpha);
    }

    pub fn clear(&self, color: Color) {
        pyxel::screen().clear(color);
    }

    pub fn get_pixel(&self, x: f32, y: f32) -> Color {
        pyxel::screen().get_pixel(x, y)
    }

    pub fn set_pixel(&self, x: f32, y: f32, color: Color) {
        pyxel::screen().set_pixel(x, y, color);
    }

    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
        pyxel::screen().draw_line(x1, y1, x2, y2, color);
    }

    pub fn draw_rect(&self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        pyxel::screen().draw_rect(x, y, width, height, color);
    }

    pub fn draw_rect_border(&self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        pyxel::screen().draw_rect_border(x, y, width, height, color);
    }

    pub fn draw_circle(&self, x: f32, y: f32, radius: f32, color: Color) {
        pyxel::screen().draw_circle(x, y, radius, color);
    }

    pub fn draw_circle_border(&self, x: f32, y: f32, radius: f32, color: Color) {
        pyxel::screen().draw_circle_border(x, y, radius, color);
    }

    pub fn draw_ellipse(&self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        pyxel::screen().draw_ellipse(x, y, width, height, color);
    }

    pub fn draw_ellipse_border(&self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        pyxel::screen().draw_ellipse_border(x, y, width, height, color);
    }

    pub fn draw_triangle(
        &self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        color: Color,
    ) {
        pyxel::screen().draw_triangle(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn draw_triangle_border(
        &self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        color: Color,
    ) {
        pyxel::screen().draw_triangle_border(x1, y1, x2, y2, x3, y3, color);
    }

    pub fn flood_fill(&self, x: f32, y: f32, color: Color) {
        pyxel::screen().flood_fill(x, y, color);
    }

    pub fn draw_image(
        &self,
        x: f32,
        y: f32,
        image_index: u32,
        image_x: f32,
        image_y: f32,
        width: f32,
        height: f32,
        color_key: Option<Color>,
        rotate: Option<f32>,
        scale: Option<f32>,
    ) {
        unsafe {
            pyxel::screen().draw_image(
                x,
                y,
                pyxel::images()[image_index as usize],
                image_x,
                image_y,
                width,
                height,
                color_key,
                rotate,
                scale,
            );
        }
    }

    pub fn draw_tilemap(
        &self,
        x: f32,
        y: f32,
        tilemap_index: u32,
        tilemap_x: f32,
        tilemap_y: f32,
        width: f32,
        height: f32,
        color_key: Option<Color>,
        rotate: Option<f32>,
        scale: Option<f32>,
    ) {
        unsafe {
            pyxel::screen().draw_tilemap(
                x,
                y,
                pyxel::tilemaps()[tilemap_index as usize],
                tilemap_x,
                tilemap_y,
                width,
                height,
                color_key,
                rotate,
                scale,
            );
        }
    }

    pub fn draw_text(&self, x: f32, y: f32, string: &str, color: Color, font: Option<*mut Font>) {
        pyxel::screen().draw_text(x, y, string, color, font);
    }

    pub(crate) fn render_screen(&mut self) {
        unsafe {
            let gl = platform::gl_context();
            self.set_viewport(gl);
            self.use_screen_shader(gl);
            self.bind_screen_texture(gl);
            self.bind_colors_texture(gl);
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }
    }

    unsafe fn set_viewport(&self, gl: &mut glow::Context) {
        let (window_width, window_height) = platform::window_size();
        gl.viewport(0, 0, window_width as i32, window_height as i32);
    }

    unsafe fn use_screen_shader(&self, gl: &mut glow::Context) {
        let shader = &self.graphics.screen_shaders[self.system.screen_mode as usize];
        gl.use_program(Some(shader.shader_program));
        let uniform_locations = &shader.uniform_locations;

        if let Some(location) = uniform_locations.get("u_screenPos") {
            let (_, window_height) = platform::window_size();
            gl.uniform_2_f32(
                Some(location),
                self.system.screen_x as f32,
                (window_height as i32
                    - self.system.screen_y
                    - (*pyxel::height() as f32 * self.system.screen_scale) as i32)
                    as f32,
            );
        }

        if let Some(location) = uniform_locations.get("u_screenSize") {
            gl.uniform_2_f32(
                Some(location),
                *pyxel::width() as f32 * self.system.screen_scale,
                *pyxel::height() as f32 * self.system.screen_scale,
            );
        }

        if let Some(location) = uniform_locations.get("u_screenScale") {
            gl.uniform_1_f32(Some(location), self.system.screen_scale);
        }

        if let Some(location) = uniform_locations.get("u_numColors") {
            gl.uniform_1_i32(Some(location), pyxel::colors().len() as i32);
        }

        if let Some(location) = uniform_locations.get("u_backgroundColor") {
            gl.uniform_3_f32(
                Some(location),
                ((BACKGROUND_COLOR >> 16) as u8) as f32 / 255.0,
                ((BACKGROUND_COLOR >> 8) as u8) as f32 / 255.0,
                (BACKGROUND_COLOR as u8) as f32 / 255.0,
            );
        }

        if let Some(location) = uniform_locations.get("u_screenTexture") {
            gl.uniform_1_i32(Some(location), 0);
        }

        if let Some(location) = uniform_locations.get("u_colorsTexture") {
            gl.uniform_1_i32(Some(location), 1);
        }

        gl.bind_vertex_array(Some(shader.vertex_array));
    }

    unsafe fn bind_screen_texture(&self, gl: &mut glow::Context) {
        gl.active_texture(glow::TEXTURE0);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.graphics.screen_texture));
        gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);

        let (internal_format, format) = if platform::gl_profile() == GLProfile::Gles {
            (glow::LUMINANCE as i32, glow::LUMINANCE)
        } else {
            (glow::R8 as i32, glow::RED)
        };

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            internal_format,
            *pyxel::width() as i32,
            *pyxel::height() as i32,
            0,
            format,
            glow::UNSIGNED_BYTE,
            PixelUnpackData::Slice(Some(&pyxel::screen().canvas.data)),
        );
    }

    unsafe fn bind_colors_texture(&self, gl: &mut glow::Context) {
        gl.active_texture(glow::TEXTURE1);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.graphics.colors_texture));
        gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 4);

        let colors = pyxel::colors();
        assert!(
            !colors.is_empty() && colors.len() <= MAX_COLORS as usize,
            "Number of colors must be between 1 to {MAX_COLORS}",
        );

        let mut pixels: Vec<u8> = Vec::with_capacity(colors.len() * 3);
        for color in colors.iter() {
            pixels.push((color >> 16) as u8);
            pixels.push((color >> 8) as u8);
            pixels.push(*color as u8);
        }

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGB as i32,
            colors.len() as i32,
            1,
            0,
            glow::RGB,
            glow::UNSIGNED_BYTE,
            PixelUnpackData::Slice(Some(&pixels)),
        );
    }
}
