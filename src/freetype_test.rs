#[cfg(target_os = "windows")]
static FONT_PATH: &'static str = "C:\\Users\\Haloc\\Dev\\Rust\\plot-rs\\cambriaz.ttf";
#[cfg(target_os = "linux")]
static FONT_PATH: &'static str = "/home/hussein/Dev/Rust/plotters/cambriaz.ttf";

use std::{
    alloc::{alloc, Layout},
    time::{Duration, Instant},
};

use freetype::{self, face::LoadFlag, Face, GlyphMetrics, GlyphSlot, Library};
use image::{GrayImage, Luma};
use nalgebra_glm::{Vec2, Vec3};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::{Camera, CameraProjection, CameraView},
    shader_builder::{
        shader::{CompileProgram, CompileProgramFromString},
        ColorAssignment, ShaderAttribute, ShaderAttributeType, ShaderBuilder,
    },
    shaders,
    textures::Texture,
    window::Window,
};

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
    pub fn conv266_to_i32(int266: i64) -> i32 {
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

struct TextGenerator2 {
    characters: Vec<FreeTypeCharacter>,
}
impl TextGenerator2 {
    fn new(face: &Face, size: u32) -> Self {
        Self {
            characters: (0..256)
                .into_iter()
                .map(|i| FreeTypeCharacter::new_character(&face, size, char::from_u32(i).unwrap()))
                .collect(),
        }
    }
    fn generate_text<'a>(&self, text: &'a str) -> (GrayImage, i32, i32) {
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

    fn generate_normalized_atlas(&self) -> FreeTypeAtlas {
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

        let height = row_heights.iter().sum::<i32>() as u32;
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

                current_pos.0 += current_char.metrics.width;
            }
            current_pos.1 += row_heights[r];
            current_pos.0 = 0;
        }

        FreeTypeAtlas {
            atlas: atlas_image,
            metrics_vector: metrics_vector,
        }
    }
}

struct FreeTypeAtlas {
    atlas: GrayImage,
    metrics_vector: Vec<(f32, f32, CharMetrics<f32>)>,
}
impl FreeTypeAtlas {
    // fn generate_texture(&self)->Texture{

    // }

    pub fn generate_text<'a>(&self, text: &'a str, size: f32) -> Vec<(Vec3, Vec3, Vec2)> {
        let start_position = Vec3::zeros();

        let mut vertices: Vec<(Vec3, Vec3, Vec2)> = Vec::new();

        let mut advance = 0.0f32;
        for (idx, ch) in text.chars().enumerate() {
            let X = Vec3::new(1.0, 0.0, idx as f32 / 100.0) * size;
            let Y = Vec3::new(0.0, 1.0, idx as f32 / 100.0) * size;
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

use glfw::Context;
fn main() {
    let mut window = Window::default();

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunci(0, gl::ONE, gl::ONE);
        gl::BlendFunci(1, gl::ZERO, gl::ONE_MINUS_SRC_COLOR);
    }

    let ft_lib = Library::init().unwrap();
    let face = ft_lib.new_face(FONT_PATH, 0).unwrap();

    let text_gen = TextGenerator2::new(&face, 100);

    let atlas = text_gen.generate_normalized_atlas();
    let text = atlas.generate_text("f(x)=x^2", 5.0);
    println!("{:?}", text);
    let mut shader = ShaderBuilder::empty();
    shader
        .set_vertex_shader_input_attributes(Some(
            [
                ShaderAttribute::new_input_attribute(
                    Some(0),
                    "position".to_string(),
                    ShaderAttributeType::Vec4,
                ),
                ShaderAttribute::new_input_attribute(
                    Some(1),
                    "color".to_string(),
                    ShaderAttributeType::Vec4,
                ),
                ShaderAttribute::new_input_attribute(
                    Some(2),
                    "txcoord".to_string(),
                    ShaderAttributeType::Vec2,
                ),
            ]
            .to_vec(),
        ))
        .add_vertex_uniform(ShaderAttribute::generate_uniform(
            None,
            "Proj".to_string(),
            ShaderAttributeType::Mat4,
        ))
        .add_vertex_uniform(ShaderAttribute::generate_uniform(
            None,
            "View".to_string(),
            ShaderAttributeType::Mat4,
        ))
        .set_vertex_transformation(plot_rs::shader_builder::VertexTransformation::Custom(
            "gl_Position=Proj*View*vec4(position.xyz,2);".to_string(),
        ))
        .add_vertex2fragment_link(
            "txcoord".to_string(),
            ShaderAttribute::new_output_attribute(
                None,
                "txc".to_string(),
                ShaderAttributeType::Vec2,
            ),
        )
        .add_fragment_uniform(ShaderAttribute::generate_uniform(
            None,
            "TX".to_string(),
            ShaderAttributeType::Custom("sampler2D"),
        ))
        .set_color_assignment(ColorAssignment::Custom(
            r"
            float tmp = texture(TX,txc).r;
            Color = vec4(tmp);
            "
            .to_string(),
        ));

    let prog = shader.compile_shader();
    unsafe { gl::UseProgram(prog.get_id()) };
    // println!(
    //     "{}\n\n\n{}",
    //     shader.generate_vertex_shader(),
    //     shader.generate_fragment_shader(),

    // );
    let mut buffer = ArrayBuffer::default_buffer_from_slice(
        &text,
        None,
        &[
            GLAttrib::new(0, 3, 8, 0),
            GLAttrib::new(1, 3, 8, 3),
            GLAttrib::new(2, 2, 8, 6),
        ]
        .to_vec(),
        true,
    );

    let texture = Texture::new_mono(
        atlas.atlas.width() as i32,
        atlas.atlas.height() as i32,
        atlas.atlas.as_ptr() as _,
    );
    texture.set_active_texture(0).bind_texture();

    let mut camera = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );
    camera
        .get_camera_view()
        .set_eye(&Vec3::new(0.0, -1.0, 1.0))
        .update_matrix();
    camera.add_program(prog.get_id()).update_uniforms(None);
    prog.use_shader();
    buffer.vao.bind();
    buffer.vbo.bind();

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            window.m_glfw.poll_events();

            for (_, event) in glfw::flush_messages(&window.m_receiver) {
                Camera::update_camera(&event, &mut camera);
            }
            // buffer.vao.bind();
            // buffer.vbo.bind();

            gl::DrawArrays(gl::TRIANGLES, 0, 30);
        }

        window.m_window.swap_buffers();
    }
}

fn main2() {
    let freetype_library = freetype::library::Library::init().unwrap();
    let face = freetype_library.new_face(FONT_PATH, 0).unwrap();

    let chars = TextGenerator2::new(&face, 100);
    let mut char_counter = 0;
    let a = Instant::now();
    // println!("{:?}", chars.generate_text("Hello World"));

    for i in 0..100 {
        let t = i as f32 / 99.0;
        let tmp = chars.generate_text(format!("f({t})=sin=sin=sin=sin({t})").as_str());
        char_counter += tmp.1;
    }

    let atlas = chars.generate_normalized_atlas();
    println!("{:?}", atlas.metrics_vector[64]);

    // .0
    // .save("tmp.jpg")
    // .unwrap();

    // for c in chars.characters {
    //     println!(
    //         "{}:\n{:?}\n=====================================\n",
    //         c.character, c.glyph
    //     );
    // }

    let b = Instant::now();

    println!("{:?}", (b - a, char_counter))
}
