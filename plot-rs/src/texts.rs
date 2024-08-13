use std::ffi::c_long;

use freetype::{face::LoadFlag, Face, GlyphMetrics, Library};
use image::{GrayImage, Luma};
use nalgebra_glm::{Vec2, Vec3, Vec4};

use crate::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    shader_builder::RawShader,
    textures::Texture,
};

pub struct FreeTypeGenerator {
    library: freetype::Library,
    face: freetype::Face,
}

impl FreeTypeGenerator {
    pub fn new(lib: Library, font: Option<&str>) -> Self {
        let res_font = if let Some(font_path) = font {
            lib.new_face(font_path, 0)
        } else {
            lib.new_face("C:\\Users\\Haloc\\Dev\\Rust\\plot-rs\\cambriaz.ttf", 0)
        };
        /*  font.unwrap_or(
            lib.new_face(
                "/home/hussein/Dev/Rust/math-gl/LatinmodernmathRegular-z8EBa.otf",
                0,
            )
            .unwrap(),
        ); */
        Self {
            library: lib,
            face: res_font.expect("No font found !"),
        }
    }
    pub fn new_from_path(lib: Library, font: &str) -> Self {
        let res_font = lib
            .new_face(
                font, //"/home/hussein/Dev/Rust/math-gl/LatinmodernmathRegular-z8EBa.otf",
                0,
            )
            .unwrap();

        Self {
            library: lib,
            face: res_font,
        }
    }
    pub fn set_font_size(&self, font_size: u32) -> &Self {
        let _ = self.face.set_pixel_sizes(0, font_size);
        self
    }
    pub fn set_font(&mut self, font: Face) -> &mut Self {
        self.face = font;
        self
    }

    // pub fn allocate_image_for_text(text: &str, font: &Face) -> (GrayImage, i64) {
    //     let mut full_width = 0;
    //     let mut y_lower = 10000;
    //     let mut y_higher = 0;

    //     for ch in text.chars() {
    //         font.load_char(ch as _, LoadFlag::RENDER).unwrap();
    //         let glyph_metrics = font.glyph().metrics();

    //         full_width += glyph_metrics.horiAdvance >> 6;
    //         y_higher = y_higher.max(glyph_metrics.horiBearingY);

    //         y_lower = y_lower.min(glyph_metrics.height - glyph_metrics.horiBearingY);
    //     }
    //     y_higher = y_higher >> 6;
    //     y_lower = y_lower >> 6;
    //     let full_height = (y_higher) - (y_lower);

    //     (GrayImage::new(full_width as _, full_height as _), y_higher)
    // }

    pub fn generate_image(&self, text: &str) -> GrayImage {
        let mut x_right = 0;

        let mut y0 = 0i32;
        let mut y1 = 0i32;
        for ch in text.chars() {
            self.face.load_char(ch as _, LoadFlag::RENDER);
            let char_metrics = self.face.glyph().metrics();
            x_right += char_metrics.horiAdvance >> 6;
            y0 = y0.max((char_metrics.horiBearingY >> 6) as i32);
            y1 =
                y1.max((char_metrics.height >> 6) as i32 - (char_metrics.horiBearingY >> 6) as i32);
        }
        let y_down = y0 + y1;

        let mut img = GrayImage::new(x_right as _, y_down as _);
        img.fill(255);
        let mut advances = 0u32;
        for ch in text.chars() {
            self.face.load_char(ch as _, LoadFlag::RENDER).unwrap();
            let glyph_bitmap = self.face.glyph().bitmap();
            let glyph_metrics = self.face.glyph().metrics();

            let rows = glyph_bitmap.rows() as u32;
            let cols = glyph_bitmap.width() as u32;

            for r in 0..rows {
                for c in 0..cols {
                    img.put_pixel(
                        advances + c,
                        y0 as u32 - (glyph_metrics.horiBearingY >> 6) as u32 + r,
                        Luma([255
                            - glyph_bitmap.buffer()
                                [(r * glyph_bitmap.width() as u32 + c) as usize]]),
                    );
                }
            }
            advances += (glyph_metrics.horiAdvance >> 6) as u32;
        }
        println!("{:?}", (x_right, y_down));

        img
    }

    pub fn text2image(text: &str, font: Face, font_size: u32) -> GrayImage {
        font.set_pixel_sizes(0, font_size).unwrap();

        let mut full_width = 0;
        let mut y_lower = 10000;
        let mut y_higher = 0;

        for ch in text.chars() {
            font.load_char(ch as _, LoadFlag::RENDER).unwrap();
            let glyph_metrics = font.glyph().metrics();

            full_width += glyph_metrics.horiAdvance >> 6;
            y_higher = y_higher.max(glyph_metrics.horiBearingY);

            y_lower = y_lower.min(glyph_metrics.height - glyph_metrics.horiBearingY);
        }
        y_higher = y_higher >> 6;
        y_lower = y_lower >> 6;
        let full_height = (y_higher) - (y_lower);
        let mut image = GrayImage::new(200u32 + full_width as u32, 200u32 + full_height as u32);

        let mut advances = 0u32;
        for ch in text.chars() {
            font.load_char(ch as _, LoadFlag::RENDER).unwrap();
            let glyph_bitmap = font.glyph().bitmap();
            let glyph_metrics = font.glyph().metrics();
            let rows = glyph_bitmap.rows() as u32;
            let cols = glyph_bitmap.width() as u32;

            for r in 0..rows {
                for c in 0..cols {
                    image.put_pixel(
                        advances + c,
                        y_higher as u32 - (glyph_metrics.horiBearingY >> 6) as u32 + r,
                        Luma([
                            glyph_bitmap.buffer()[(r * glyph_bitmap.width() as u32 + c) as usize]
                        ]),
                    );
                }
            }
            advances += (glyph_metrics.horiAdvance >> 6) as u32;
        }

        println!("{:?}", (full_width, full_height));
        return image;
    }
}

pub struct Textures {
    texts: Vec<Texture>,
    vertices: Vec<(Vec3, Vec3, Vec2)>,
    program: RawShader,
    freetype_ctx: Option<FreeTypeGenerator>,
}

impl Textures {
    fn vec4_to_vec3(input: Vec4) -> Vec3 {
        Vec3::new(input.x, input.y, input.z)
    }
    fn generate_texture_quad(
        width: f32,
        height: f32,
        center: Option<Vec3>,
        direction: Option<Vec3>,
    ) -> Vec<(Vec3, Vec3, Vec2)> {
        let dir = nalgebra_glm::normalize(&direction.unwrap_or(Vec3::new(1.0, 0.0, 0.0)));
        let right = nalgebra_glm::normalize(&nalgebra_glm::cross(&Vec3::new(0.0, 1.0, 0.0), &dir));
        let up = -1.0 * nalgebra_glm::cross(&dir, &right);
        // y = nalgebra_glm::cross(&z, &x);

        let base_quad = [
            (
                Vec4::new(-width / 2.0, -height / 2.0, 0.0, 1.0),
                Vec2::new(0.0, 1.0),
            ),
            (
                Vec4::new(width / 2.0, height / 2.0, 0.0, 1.0),
                Vec2::new(1.0, 0.0),
            ),
            (
                Vec4::new(-width / 2.0, height / 2.0, 0.0, 1.0),
                Vec2::new(0.0, 0.0),
            ),
            (
                Vec4::new(-width / 2.0, -height / 2.0, 0.0, 1.0),
                Vec2::new(0.0, 1.0),
            ),
            (
                Vec4::new(width / 2.0, -height / 2.0, 0.0, 1.0),
                Vec2::new(1.0, 1.0),
            ),
            (
                Vec4::new(width / 2.0, height / 2.0, 0.0, 1.0),
                Vec2::new(1.0, 0.0),
            ),
        ];

        let quad_center = center.unwrap_or(Vec3::zeros());

        let (x, y, z) = (right, up, dir);

        let mut transformation = nalgebra_glm::Mat4::new(
            x.x,
            y.x,
            z.x,
            quad_center.x,
            x.y,
            y.y,
            z.y,
            quad_center.y,
            x.z,
            y.z,
            z.z,
            quad_center.z,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        //let transformation= nalgebra_glm::rotation(angle, v)
        let transformed_quad: Vec<_> = base_quad
            .iter()
            .map(|val| {
                (
                    Self::vec4_to_vec3(transformation * val.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec2::new(val.1.x, 1.0 - val.1.y),
                )
            })
            .collect();

        return transformed_quad;
    }
    // pub fn new() -> Self {
    //     // let shader = ShaderCreator::new(ColorMode::ColorTexture, false);
    //     // Self {
    //     //     texts: Vec::new(),
    //     //     vertices: Vec::new(),
    //     //     program: BasicShader::new(shader.program_id),
    //     //     freetype_ctx: None,
    //     // }
    // }

    pub fn add_text(
        &mut self,
        text_line: &str,
        size: f32,
        font_size: u32,
        center: Option<Vec3>,
        direction: Option<Vec3>,
    ) -> &mut Self {
        if self.freetype_ctx.is_none() {
            self.freetype_ctx = Some(FreeTypeGenerator::new(Library::init().unwrap(), None));
        }
        if let Some(ctx) = &mut self.freetype_ctx {
            let img = ctx.set_font_size(font_size).generate_image(text_line);
            let quad = Self::generate_texture_quad(
                size * (img.width() as f32 / img.height() as f32),
                size,
                center,
                direction,
            );
            self.vertices.extend(quad);

            let texture = Texture::new_mono(img.width() as _, img.height() as _, img.as_ptr() as _);
            texture.set_active_texture(0).bind_texture();
            self.texts.push(texture);
        }

        self
    }

    pub fn update_array_buffer(&mut self) -> &mut Self {
        // self.program.buffer = ArrayBuffer::new();
        // self.program
        //     .buffer
        //     .allocateBuffer(
        //         self.vertices.as_ptr() as _,
        //         ((8 * 4) * self.vertices.len()) as _,
        //     )
        //     .setAttributes(
        //         &[
        //             GLAttrib::new(0, 3, 8, 0),
        //             GLAttrib::new(1, 3, 8, 3),
        //             GLAttrib::new(3, 2, 8, 6),
        //         ]
        //         .to_vec(),
        //     );

        self
    }
    pub fn draw_textures(&self) {
        // unsafe {
        //     gl::UseProgram(self.program.program);
        //     self.program.buffer.bindAB();
        //     for i in 0..self.texts.len() {
        //         gl::BindTexture(gl::TEXTURE_2D, self.texts[i].get_all().0);
        //         gl::DrawArrays(gl::TRIANGLES, (i * 6) as _, 6);
        //     }
        // }
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

pub struct TextGenerator2 {
    characters: Vec<FreeTypeCharacter>,
}
impl TextGenerator2 {
    pub fn new(face: &Face, size: u32) -> Self {
        Self {
            characters: (0..256)
                .into_iter()
                .map(|i| FreeTypeCharacter::new_character(&face, size, char::from_u32(i).unwrap()))
                .collect(),
        }
    }
    pub fn generate_text<'a>(&self, text: &'a str) -> (GrayImage, i32, i32) {
        let (mut width_l, mut width_r) = (0, 0);
        let (mut height_t, mut height_b) = (0, 0);
        let mut advance = 0;
        for (idx, ch) in text.chars().enumerate() {
            let current_char = &self.characters[ch as usize];
            let mertics = current_char.metrics;

            let mut current_start = advance + mertics.horiBearingX;
            width_l = width_l.min(current_start);
            width_r = width_r.max(advance + mertics.horiAdvance);

            height_t = height_t.max(mertics.horiBearingY);
            height_b = height_b.max(mertics.height - mertics.horiBearingY);

            advance += mertics.horiAdvance;
        }

        let width = width_r - width_l;
        let height = height_t + height_b;

        advance = 0;

        let mut dest = GrayImage::new(width as u32, height as u32);

        for (idx, ch) in text.chars().enumerate() {
            let current_char = &self.characters[ch as usize];
            let mertics = current_char.metrics;
            let mut pixel_ptr = current_char.bitmap.as_ptr();
            for r in 0..mertics.height {
                for c in 0..mertics.width {
                    let mut current_pixel = dest.get_pixel_mut(
                        (advance - width_l + mertics.horiBearingX + c) as u32,
                        (height_t - mertics.horiBearingY + r) as u32,
                    );

                    *current_pixel = Luma {
                        0: [
                            (current_pixel.0[0] as u32 + unsafe { *pixel_ptr } as u32).min(255)
                                as u8,
                        ],
                    };
                    unsafe { pixel_ptr = pixel_ptr.add(1) };
                }
            }
            advance += mertics.horiAdvance;
        }

        (dest, width_r - width_l, height_t + height_b)
    }

   pub  fn generate_normalized_atlas(&self) -> FreeTypeAtlas {
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

        let height = row_heights.iter().sum::<i32>() as u32+16;
        width+=16;
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

                current_pos.0 += current_char.metrics.width+1;
            }
            current_pos.1 += row_heights[r]+1;
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
    pub fn get_atlas_image(&self)->&GrayImage{
        &self.atlas
    }
    pub fn get_metrics_vector(&self)->&Vec<(f32,f32,CharMetrics<f32>)>{
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
