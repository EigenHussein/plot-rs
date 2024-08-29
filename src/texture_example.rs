use std::any::Any;

use freetype::{face::LoadFlag, Face, Library};
use glfw::{Action, Context, WindowEvent};
use image::{DynamicImage, GrayImage, Luma};
use nalgebra_glm::{Vec2, Vec4};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    shader_builder::{ColorAssignment, ShaderAttribute, ShaderAttributeType, ShaderBuilder},
    textures::Texture,
    window::Window,
};
pub fn get_quad() -> [(Vec4, Vec4, Vec2); 4] {
    let quad_vertices = [
        (
            Vec4::new(-1.0, -1.0, 0.0, 1.0),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ),
        (
            Vec4::new(-1.0, 1.0, 0.0, 1.0),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec2::new(0.0, 0.0),
        ),
        (
            Vec4::new(1.0, 1.0, 0.0, 1.0),
            Vec4::new(0.0, 1.0, 0.0, 1.0),
            Vec2::new(1.0, 0.0),
        ),
        (
            Vec4::new(1.0, -1.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec2::new(1.0, 1.0),
        ),
    ];
    quad_vertices
}

fn conv266_to_i32(int266: i64) -> i64 {
    println!("{}: {:b}", int266, int266);
    let abs_val = (int266 & 0x7FFFFFFF_FFFFFFFF) >> 6;
    let sign = if int266 as u64 & 0x80000000_00000000 != 0 {
        -1
    } else {
        1
    };
    return abs_val * sign;
}

fn text2image(text: &str, font: Face, font_size: u32) -> GrayImage {
    font.set_pixel_sizes(0, font_size);

    let mut x_right = 0;

    let mut y0 = 0i32;
    let mut y1 = 0i32;
    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER);
        let char_metrics = font.glyph().metrics();
        x_right += char_metrics.horiAdvance - char_metrics.horiBearingX;
        y0 = y0.max((char_metrics.horiBearingY >> 6) as i32);
        y1 = y1.max((char_metrics.height >> 6) as i32 - (char_metrics.horiBearingY >> 6) as i32);
        // let current_down = 2 * (char_metrics.height >> 6) - (char_metrics.horiBearingY >> 6);
        // if current_down > y_down {
        //     y_down = current_down;
        //     origin=y_down -((char_metrics.height>>6) - (char_metrics.horiBearingY >> 6) );
        // }
        // y_down = y_down.max();
    }
    let y_down = y0 + y1;

    let mut img = GrayImage::new((x_right >> 6) as u32 + 100, y_down as u32 + 100);
    let mut advances = 0u32;
    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER).unwrap();
        let glyph_bitmap = font.glyph().bitmap();
        let glyph_metrics = font.glyph().metrics();
        let rows = glyph_bitmap.rows() as u32;
        let cols = glyph_bitmap.width() as u32;

        print!(
            r"
            Width: {}
            Height: {}
            Advance: {}
            xBearing: {}
            yBearing: {}
            len: {}
            ",
            /* conv266_to_i32 */ (glyph_metrics.width),
            /* conv266_to_i32 */ (glyph_metrics.height),
            /* conv266_to_i32 */ (glyph_metrics.horiAdvance),
            /* conv266_to_i32 */ (glyph_metrics.horiBearingX),
            /* conv266_to_i32 */ (glyph_metrics.horiBearingY),
            /* conv266_to_i32 */ (glyph_metrics.horiAdvance - glyph_metrics.horiBearingX)
        );

        for r in 0..rows {
            for c in 0..cols {
                img.put_pixel(
                    advances + c,
                    y0 as u32 - (glyph_metrics.horiBearingY >> 6) as u32 + r,
                    Luma([glyph_bitmap.buffer()[(r * glyph_bitmap.width() as u32 + c) as usize]]),
                );
            }
        }
        advances += (glyph_metrics.horiAdvance >> 6) as u32;
    }
    println!("{:?}", (x_right, y_down));

    img
}
#[cfg(target_os = "windows")]
static FONT_PATH: &'static str = "C:\\Users\\Haloc\\Dev\\Rust\\plot-rs\\cambriaz.ttf";
#[cfg(target_os = "linux")]
static FONT_PATH: &'static str = "/home/hussein/Dev/Rust/plotters/cambriaz.ttf";

//testing text generation

fn text2image2(text: &str, font: Face, font_size: u32) -> GrayImage {
    font.set_pixel_sizes(0, font_size);
    let mut texture_width = 0;
    let mut texture_height = 0;
    let mut yl = -100000;
    let mut yh = -100000;
    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER).unwrap();
        let glyph_metrics = font.glyph().metrics();
        texture_width += glyph_metrics.horiAdvance; //+ glyph_metrics.horiBearingX;
        yh = yh.max(glyph_metrics.horiBearingY);
        yl = yl.max(glyph_metrics.height - glyph_metrics.horiBearingY) //yl.max(-glyph_metrics.horiBearingY);
    }

    texture_width = (texture_width >> 6) + 40;
    texture_height = (yh + yl) >> 6;

    // {
    //     let mut advance = 0;
    //     for ch in text.chars() {
    //         font.load_char(ch as _, LoadFlag::RENDER).unwrap();
    //         let glyph_metrics = font.glyph().metrics();
    //         let glyph_bitmap = font.glyph().bitmap();
    //         let c0 = ((advance + glyph_metrics.horiBearingX) >> 6) as i32;
    //         let r0 = ((yh - glyph_metrics.horiBearingY) >> 6) as i32;
    //         let cols = glyph_bitmap.width();
    //         let rows = glyph_bitmap.rows();

    //         for r in 0..rows as i32 {
    //             for c in 0..cols as i32 {
    //                 let ptr =
    //                     &mut texture_image[((c + c0) as u32, (r + r0) as u32)] as *mut _ as *mut u8;
    //                 unsafe {
    //                     *ptr += glyph_bitmap.buffer()[(r * glyph_bitmap.width() + c) as usize]
    //                 };
    //                 // = Luma {
    //                 //     0: [glyph_bitmap.buffer()[(r * glyph_bitmap.width() + c) as usize]],
    //                 // };
    //             }
    //         }

    //         advance += glyph_metrics.horiAdvance;
    //     }
    //     println!("{:?}", (texture_width, texture_height));
    // }

    let mut texture_image = GrayImage::new(texture_width as u32, texture_height as u32);

    let mut advance = 0;
    for ch in text.chars() {
        font.load_char(ch as _, LoadFlag::RENDER).unwrap();
        let glyph_metrics = font.glyph().metrics();
        let glyph_bitmap = font.glyph().bitmap();
        let c0 = ((advance + glyph_metrics.horiBearingX) >> 6) as i32;
        let r0 = ((yh - glyph_metrics.horiBearingY) >> 6) as i32;
        let cols = glyph_bitmap.width();
        let rows = glyph_bitmap.rows();

        for r in 0..rows as i32 {
            for c in 0..cols as i32 {
                let ptr =
                    &mut texture_image[((c + c0) as u32, (r + r0) as u32)] as *mut _ as *mut u8;
                unsafe { *ptr += glyph_bitmap.buffer()[(r * glyph_bitmap.width() + c) as usize] };
                // = Luma {
                //     0: [glyph_bitmap.buffer()[(r * glyph_bitmap.width() + c) as usize]],
                // };
            }
        }

        advance += glyph_metrics.horiAdvance;
    }
    println!("{:?}", (texture_width, texture_height));
    texture_image
}

fn main2() {
    let ft_lib = Library::init().unwrap();
    let face = ft_lib.new_face(FONT_PATH, 0).unwrap();
 
    let img = text2image2("  f(x=100)", face, 120); //text2image("Hello, World", face, 20);
    img.save("tmp.jpg");
}

fn main() {
    let mut window = Window::default();

    let ft_lib = Library::init().unwrap();
    let face = ft_lib.new_face(FONT_PATH, 0).unwrap();
 

    let img = FreeTypeGenerator::text2image("HelloWorld", face, 20);

    // let text_gen = FreeTypeGenerator::new(Library::init().unwrap(), None);
    // let face = text_gen
    // let img = text_gen.generate_image("Hello World");

    // for i in 0..100usize {
    //     for j in 0..100usize {
    //         println!("{}", unsafe { *img.as_ptr().add(100 * i + j) });
    //     }
    // }
    let quad = get_quad();

    let ab = ArrayBuffer::default_buffer_from_slice(
        &quad,
        Some(&[0, 2, 1, 0, 3, 2]),
        &[
            GLAttrib::new(0, 4, 10, 0),
            GLAttrib::new(1, 4, 10, 4),
            GLAttrib::new(2, 2, 10, 8),
        ]
        .to_vec(),
        true,
    );

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
            "Color = vec4(vec3(texture(TX,txc).r),1);".to_string(),
        ));

    let shade = shader.compile_shader();
    println!(
        "{}\n\n\n{}",
        shader.generate_fragment_shader(),
        shader.generate_vertex_shader()
    );
    let mut texture = 0;

    // let data: Vec<_> = (0..(1024 * 1024)).map(|i| (i % 255) as u8).collect(); // [[128u8;1024];1024];
    // let texture = Texture::new_mono(1024 as _, 1024 as _, data.as_ptr() as _);

    let texture = Texture::new_mono(img.width() as _, img.height() as _, img.as_ptr() as _);
    texture.bind_texture().set_active_texture(0);
    // unsafe {
    //     // gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
    //     gl::GenTextures(1, &mut texture);
    //     gl::BindTexture(gl::TEXTURE_2D, texture);
    //     Texture::texture_set_default_parameters();
    //     gl::TexImage2D(
    //         gl::TEXTURE_2D,
    //         0,
    //         gl::R32F as _,
    //         1024 as _,
    //         1024 as _,
    //         0,
    //         gl::RED,
    //         gl::UNSIGNED_BYTE,
    //         data.as_ptr() as _,
    //     );
    //     // gl::ActiveTexture(gl::TEXTURE0);
    //     // gl::BindTexture(gl::TEXTURE_2D, texture);
    // }

    let mut cam = Camera::default();
    cam.add_program(shade.get_id()).update_uniforms(None);
    ab.vao.bind();
    ab.ebo.bind();
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(shade.get_id());
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
        }

        window.m_window.swap_buffers();
    }
}

fn update_camera(ev: &WindowEvent, camera_instance: &mut Camera) {
    match ev {
        WindowEvent::Key(key, scancode, action, modifier) => {
            let act = *action;
            if *action == Action::Repeat || *action == Action::Press {
                match *key {
                    glfw::Key::A => {
                        camera_instance
                            .get_camera_view()
                            .increase_azimuth(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms(None);
                    }
                    glfw::Key::D => {
                        camera_instance
                            .get_camera_view()
                            .decrease_azimuth(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms(None);
                    }
                    glfw::Key::W => {
                        camera_instance
                            .get_camera_view()
                            .increase_altitude(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms(None);
                    }
                    glfw::Key::S => {
                        camera_instance
                            .get_camera_view()
                            .decrease_altitude(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms(None);
                    }

                    _ => {}
                };
            }
        }
        _ => {}
    };
}
