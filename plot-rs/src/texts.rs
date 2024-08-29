use std::ffi::c_long;

use freetype::{face::LoadFlag, Face, GlyphMetrics, Library};
use image::{GrayImage, Luma};
use nalgebra_glm::{vec3_to_vec4, Vec2, Vec3, Vec4};

use crate::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    coordinate_mapping::Coordinate,
    shader_builder::{shader::CompileProgram, RawShader},
    textures::Texture,
};

pub struct TextBoxes {
    texts: Vec<Texture>,
    vertices: Vec<(Vec4, Vec4, Vec2)>,
    program: RawShader,
    buffer: ArrayBuffer,
    freetype_generator: TextGenerator,
}

impl TextBoxes {
    fn vec4_to_vec3(input: Vec4) -> Vec3 {
        Vec3::new(input.x, input.y, input.z)
    }

    fn simple_quad() -> [(Vec4, Vec4, Vec2); 6] {
        // const ZERO : f32 = 0.001;

        [
            (
                Vec4::new(-0.5, -0.5, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ),
            (
                Vec4::new(0.5, -0.5, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec2::new(1.0, 1.0),
            ),
            (
                Vec4::new(0.5, 0.5, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec2::new(1.0, 0.0),
            ),
            (
                Vec4::new(-0.5, -0.5, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ),
            (
                Vec4::new(-0.5, 0.5, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec2::new(0.0, 0.0),
            ),
            (
                Vec4::new(0.5, 0.5, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec2::new(1.0, 0.0),
            ),
        ]
    }

    fn generate_texture_quad(
        height: f32,
        aspect: f32,
        coordinates: Coordinate,
    ) -> Vec<(Vec4, Vec4, Vec2)> {
        let width = height * aspect;
        let mut quad = Self::simple_quad();

        for (V, _, _) in quad.iter_mut() {
            V.x *= width;
            V.y *= height;
            *V = coordinates.map_4d(&V);
        }
        quad.to_vec()
    }

    pub fn new(prog_id: Option<u32>, font: Face, pixel_resolution: u32) -> Self {
        Self {
            texts: Vec::new(),
            vertices: Vec::new(),
            program: RawShader::wrap(prog_id.unwrap_or(CompileProgram(
                "shaders/textures/vertex.glsl",
                "shaders/textures/fragment.glsl",
                None,
            ))),
            buffer: ArrayBuffer::default(),
            freetype_generator: TextGenerator::new(&font, pixel_resolution),
        }
    }

    pub fn add_text(&mut self, text_line: &str, height: f32, coordinates: Coordinate) -> &mut Self {
        let text_texture = self.freetype_generator.generate_text_texture(text_line);
        let quad = Self::generate_texture_quad(
            height,
            text_texture.1 as f32 / text_texture.2 as f32,
            coordinates,
        );
        let texture = Texture::new_mono(
            text_texture.1 as _,
            text_texture.2 as _,
            text_texture.0.as_ptr(),
        );
        texture.set_active_texture(0).bind_texture();

        self.texts.push(texture);
        self.vertices.extend(quad);
        self
    }
    pub fn clear_text(&mut self)->&mut Self{
        self.texts.clear();
        self.vertices.clear();
        self
    }

    pub fn update_array_buffer(&mut self) -> &mut Self {
        self.buffer = ArrayBuffer::default_buffer_from_slice(
            &self.vertices,
            None,
            &[
                GLAttrib::new(0, 4, 10, 0),
                GLAttrib::new(1, 4, 10, 4),
                GLAttrib::new(2, 2, 10, 8),
            ]
            .to_vec(),
            true,
        );
        self
    }
    pub fn draw_textures(&self) {
        unsafe {
            gl::UseProgram(self.program.get_id());
            self.buffer.vao.bind();
            self.buffer.vbo.bind();
            for i in 0..self.texts.len() {
                gl::BindTexture(gl::TEXTURE_2D, self.texts[i].get_all().0);
                gl::DrawArrays(gl::TRIANGLES, (i * 6) as _, 6);
            }
        }
    }
    pub fn add_camera(&mut self, camera: &mut Camera) -> &mut Self {
        camera
            .add_program(self.program.get_id())
            .update_uniforms(None);
        self
    }
}

//=============================================== Fretype 2

#[derive(Debug, Clone, Copy)]
pub struct CharMetrics<T> {
    pub width: T,
    pub height: T,

    pub horiBearingX: T,
    pub horiBearingY: T,
    pub horiAdvance: T,

    pub vertBearingX: T,
    pub vertBearingY: T,
    pub vertAdvance: T,
}

impl CharMetrics<i32> {
    pub fn conv266_to_i32(int266: c_long) -> i32 {
        // println!("{}: {:b}", int266, int266);

        let sign = if int266 < 0 { -1 } else { 1 };

        return ((int266.abs() >> 6) * sign) as i32;
    }
    pub fn from_freetype_metrics(metrics: &GlyphMetrics) -> Self {
        CharMetrics {
            width: Self::conv266_to_i32(metrics.width),
            height: Self::conv266_to_i32(metrics.height),
            horiBearingX: Self::conv266_to_i32(metrics.horiBearingX),
            horiBearingY: Self::conv266_to_i32(metrics.horiBearingY),
            horiAdvance: Self::conv266_to_i32(metrics.horiAdvance),
            vertBearingX: Self::conv266_to_i32(metrics.vertBearingX),
            vertBearingY: Self::conv266_to_i32(metrics.vertBearingY),
            vertAdvance: Self::conv266_to_i32(metrics.vertAdvance),
        }
    }
    pub fn normalize(&self, width: f32, height: f32) -> CharMetrics<f32> {
        CharMetrics {
            width: self.width as f32 / width,
            height: self.height as f32 / height,
            horiBearingX: self.horiBearingX as f32 / width as f32,
            horiBearingY: self.horiBearingY as f32 / height as f32,
            horiAdvance: self.horiAdvance as f32 / width as f32,
            vertBearingX: self.vertBearingX as f32 / width as f32,
            vertBearingY: self.vertBearingY as f32 / height as f32,
            vertAdvance: self.vertAdvance as f32 / height as f32,
        }
    }
}

struct FreeTypeCharacter {
    character: char,
    bitmap: Vec<u8>,
    metrics: CharMetrics<i32>,
}
impl FreeTypeCharacter {
    pub fn new_character(face: &Face, size: u32, character: char) -> Self {
        face.set_pixel_sizes(0, size);
        face.load_char(character as _, LoadFlag::RENDER);
        let glyph = face.glyph();
        let metrics = glyph.metrics();
        Self {
            character: character,
            bitmap: glyph.bitmap().buffer().to_vec(),
            metrics: CharMetrics::from_freetype_metrics(&glyph.metrics()),
        }
    }
}

pub struct TextGenerator {
    characters: Vec<FreeTypeCharacter>,
}
impl TextGenerator {
    pub fn new(face: &Face, size: u32) -> Self {
        Self {
            characters: (0..256)
                .into_iter()
                .map(|i| FreeTypeCharacter::new_character(&face, size, char::from_u32(i).unwrap()))
                .collect(),
        }
    }

    fn get_pixel_range<'a>(&self, text: &'a str) -> (i32, i32, i32, i32) {
        let mut x_min = 1000000;
        let mut x_max = -1000000;
        let mut y_min = 1000000;
        let mut y_max = -1000000;

        let mut advance = 0;

        // font.set_pixel_sizes(0, pixel_size).unwrap();

        for ch in text.chars() {
            // font.load_char(ch as _, LoadFlag::RENDER).unwrap();
            // let glyph = font.glyph();
            // let processed_metrics = CharMetrics::from_freetype_metrics(&glyph.metrics());
            let processed_metrics = &self.characters[ch as usize].metrics;
            // println!("{:?}", processed_metrics);
            // character's information
            let x0 = advance + processed_metrics.horiBearingX;
            let x1 = x0 + processed_metrics.width;
            let y1 = processed_metrics.horiBearingY;
            let y0 = y1 - processed_metrics.height;

            x_min = x_min.min(x0);
            x_max = x_max.max(x1);
            y_min = y_min.min(y0);
            y_max = y_max.max(y1);
            advance += processed_metrics.horiAdvance;
        }

        (x_min, x_max, y_min, y_max)
    }

    pub fn generate_text_texture<'a>(&self, text: &'a str) -> (GrayImage, u32, u32) {
        let freetype_pixel_range = self.get_pixel_range(text);

        let texture_width = (freetype_pixel_range.1 - freetype_pixel_range.0) as _;
        let texture_height = (freetype_pixel_range.3 - freetype_pixel_range.2) as _;

        let mut output_texture = GrayImage::new(texture_width, texture_height);

        let x_offset = -freetype_pixel_range.0;
        let y_offset = -freetype_pixel_range.2;

        let mut advance = 0;
        for ch in text.chars() {
            let processed_metrics = self.characters[ch as usize].metrics;
            // println!("{:?}", processed_metrics);

            let x0 = advance + processed_metrics.horiBearingX;
            let y0 = freetype_pixel_range.3 - processed_metrics.horiBearingY;

            let bitmap = self.characters[ch as usize].bitmap.as_ptr(); //let bitmap = glyph.bitmap().buffer().as_ptr();
            for y in 0..processed_metrics.height {
                for x in 0..processed_metrics.width {
                    let x_val = (x0 + x_offset + x) as u32;
                    let y_val = (y0 + y) as u32;

                    let current_pixel = &mut output_texture[(x_val, y_val)].0[0];

                    *current_pixel = *current_pixel
                        + unsafe { *bitmap.add((y * processed_metrics.width + x) as usize) };
                }
            }
            advance += processed_metrics.horiAdvance;
        }

        (output_texture, texture_width, texture_height)
    }

    pub fn generate_normalized_atlas(&self) -> FreeTypeAtlas {
        let mut width = 0;
        let mut row_heights = [0; 16];

        for r in 0..16 {
            let mut current_row_width = 0;
            for c in 0..16 {
                let current_char = &self.characters[(r * 16 + c) as usize];
                current_row_width += current_char.metrics.width;
                row_heights[r] = row_heights[r].max(current_char.metrics.height);
            }
            width = width.max(current_row_width);
        }

        let height = row_heights.iter().sum::<i32>() as u32 + 16;
        width += 16;
        let mut atlas_image = GrayImage::new(width as u32, height);
        let mut metrics_vector: Vec<(f32, f32, CharMetrics<f32>)> = Vec::with_capacity(256);

        let mut current_pos = (0, 0);

        for r in 0..16 {
            for c in 0..16 {
                let current_char = &self.characters[(r * 16 + c) as usize];
                let mut char_ptr = current_char.bitmap.as_ptr();

                metrics_vector.push((
                    current_pos.0 as f32 / width as f32,
                    current_pos.1 as f32 / height as f32,
                    current_char.metrics.normalize(width as f32, height as f32),
                ));
                for pixel_row in 0..current_char.metrics.height {
                    for pixel_col in 0..current_char.metrics.width {
                        atlas_image[(
                            (current_pos.0 + pixel_col) as u32,
                            (current_pos.1 + pixel_row) as u32,
                        )] = Luma {
                            0: [unsafe { *char_ptr }],
                        };
                        unsafe {
                            char_ptr = char_ptr.add(1);
                        }
                    }
                }

                current_pos.0 += current_char.metrics.width + 1;
            }
            current_pos.1 += row_heights[r] + 1;
            current_pos.0 = 0;
        }

        FreeTypeAtlas {
            atlas: atlas_image,
            metrics_vector: metrics_vector,
        }
    }
}

pub struct FreeTypeAtlas {
    atlas: GrayImage,
    metrics_vector: Vec<(f32, f32, CharMetrics<f32>)>,
}
impl FreeTypeAtlas {
    // fn generate_texture(&self)->Texture{

    // }
    pub fn get_atlas_image(&self) -> &GrayImage {
        &self.atlas
    }
    pub fn get_metrics_vector(&self) -> &Vec<(f32, f32, CharMetrics<f32>)> {
        &self.metrics_vector
    }
    pub fn generate_text<'a>(&self, text: &'a str, size: f32) -> Vec<(Vec3, Vec3, Vec2)> {
        let start_position = Vec3::zeros();

        let mut vertices: Vec<(Vec3, Vec3, Vec2)> = Vec::new();

        let mut advance = 0.0f32;
        for (idx, ch) in text.chars().enumerate() {
            let X = Vec3::new(1.0, 0.0, idx as f32 / 10000.0) * size;
            let Y = Vec3::new(0.0, 1.0, idx as f32 / 10000.0) * size;
            let current_char = self.metrics_vector[ch as usize];
            //  p1 ... p2
            //  .       .
            //  .       .
            //  P4 ... P3
            let p1 = (advance + current_char.2.horiBearingX) * X + current_char.2.horiBearingY * Y;
            let p2 = (advance + current_char.2.horiBearingX + current_char.2.width) * X
                + current_char.2.horiBearingY * Y;

            let p3 = (advance + current_char.2.horiBearingX + current_char.2.width) * X
                + (current_char.2.horiBearingY - current_char.2.height) * Y;
            let p4 = (advance + current_char.2.horiBearingX) * X
                + (current_char.2.horiBearingY - current_char.2.height) * Y;

            let t1 = Vec2::new(current_char.0, current_char.1);
            let t2 = Vec2::new(current_char.0 + current_char.2.width, current_char.1);
            let t3 = Vec2::new(
                current_char.0 + current_char.2.width,
                current_char.1 + current_char.2.height,
            );
            let t4 = Vec2::new(current_char.0, current_char.1 + current_char.2.height);

            vertices.extend_from_slice(&[
                (p4, Vec3::z(), t4),
                (p3, Vec3::z(), t3),
                (p2, Vec3::z(), t2),
                (p4, Vec3::z(), t4),
                (p2, Vec3::z(), t2),
                (p1, Vec3::z(), t1),
            ]);
            advance += current_char.2.horiAdvance;
        }
        vertices
    }
}




pub struct DynamicTexts {
    atlas: FreeTypeAtlas,
    atlas_texture: Texture,
    buffer: ArrayBuffer,
    program: RawShader,

    // array
    /// Text, Height, Coordinates,
    texts: Vec<(String, f32, Coordinate)>,
    capacity: usize,
    number_of_chars: usize,
}
impl DynamicTexts {

    /// ## Description
    /// Create a dynamic text generator based on font atlas
    /// 
    /// ### Example
    /// 
    /// ```
    ///     let mut tboxes = DynamicTexts::new("cambriaz.ttf", 100, 300).unwrap();
    ///     tboxes
    ///     .add_text("X Label", 1.0, &Coordinate::default())
    ///     .add_text(
    ///         "Y Label",
    ///         1.0,
    ///         &Coordinate::default()
    ///             .rotate_ccw_w(std::f32::consts::PI / 2.00)
    ///             .clone(),
    ///     )
    ///     .update_buffer()
    ///     .bind_camera(&mut cam);
    /// ```
    /// 
    /// 
    pub fn new(font_path: &str, resolution: u32, char_capacity: usize) -> Result<Self, ()> {
        let lib = freetype::Library::init().unwrap();
        if let Ok(font) = lib.new_face(font_path, 0) {
            let text_generator = TextGenerator::new(&font, resolution);
            let atlas = text_generator.generate_normalized_atlas();
            let atlas_texture_image = atlas.get_atlas_image();

            let texture = Texture::new_mono(
                atlas_texture_image.width() as _,
                atlas_texture_image.height() as _,
                atlas_texture_image.as_ptr(),
            );

            let mut array_buffer = ArrayBuffer::default_buffer_from_slice(
                &vec![(Vec4::zeros(), Vec4::zeros(), Vec2::zeros()); 6 * char_capacity],
                None,
                &[
                    GLAttrib::new(0, 4, 10, 0),
                    GLAttrib::new(1, 4, 10, 4),
                    GLAttrib::new(2, 2, 10, 8),
                ]
                .to_vec(),
                false,
            );

            let prog = RawShader::wrap(CompileProgram(
                "shaders/textures/vertex.glsl",
                "shaders/textures/fragment.glsl",
                None,
            ));

            return Ok(Self {
                atlas: text_generator.generate_normalized_atlas(),
                atlas_texture: texture,
                buffer: array_buffer,
                program: prog,
                texts: Vec::new(),
                capacity: char_capacity,
                number_of_chars: 0,
            });
        }

        Err(())
    }

    pub fn update_buffer(&mut self) -> &mut Self {
        unsafe {
            let ptr = gl::MapNamedBuffer(self.buffer.vbo.unwrap_buffer(), gl::WRITE_ONLY);
            let mapped_buffer =
                std::slice::from_raw_parts_mut(ptr as *mut (Vec4, Vec4, Vec2), self.capacity);

            let cap = self.capacity.min(self.number_of_chars);
            let mut vertex_idx = 0;

            for text_box in self.texts.iter() {
                let full_text_vertices = 6 * text_box.0.len();
                if vertex_idx + full_text_vertices > cap * 6 {
                    break;
                }

                let vertices = self.atlas.generate_text(text_box.0.as_str(), text_box.1);
                for i in 0..full_text_vertices {
                    mapped_buffer[vertex_idx + i] = (
                        text_box.2.map_3d(&vertices[i].0),
                        vec3_to_vec4(&vertices[i].1),
                        vertices[i].2,
                    );
                }
                vertex_idx += full_text_vertices;
            }

            gl::UnmapNamedBuffer(self.buffer.vbo.unwrap_buffer());
        }

        self
    }

    pub fn add_text<'a>(&mut self, text: &'a str, height: f32, coords: &Coordinate) -> &mut Self {
        self.texts.push((text.to_string(), height, coords.clone()));
        self.number_of_chars += text.len();
        self
    }

    pub fn bind_camera(&mut self, cam: &mut Camera) -> &mut Self {
        cam.add_program(self.program.get_id());
        self
    }
    pub fn draw_on(&self) {
        self.program.use_shader();
        self.buffer.vao.bind();
        self.buffer.vbo.bind();
        self.atlas_texture.bind_texture();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, (6 * self.number_of_chars) as _);
        }
    }
}
