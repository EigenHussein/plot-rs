use std::ffi::c_long;

use freetype::{face::LoadFlag, Face, GlyphMetrics, Library};
use image::{GrayImage, Luma};
use nalgebra_glm::{Vec2, Vec3, Vec4};

use crate::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    coordinate_mapping::Coordinate,
    shader_builder::RawShader,
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

    pub fn new(prog_id: u32, font: Face, pixel_resolution: u32) -> Self {
        Self {
            texts: Vec::new(),
            vertices: Vec::new(),
            program: RawShader::wrap(prog_id),
            buffer: ArrayBuffer::default(),
            freetype_generator: TextGenerator::new(&font, pixel_resolution),
        }
    }

    pub fn add_text(
        &mut self,
        text_line: &str,
        height: f32,
        coordinates: Coordinate,

    ) -> &mut Self {
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
            println!("{:?}", processed_metrics);
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
