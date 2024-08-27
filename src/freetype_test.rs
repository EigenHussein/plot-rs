
use std::{
    alloc::{alloc, Layout}, ffi::c_long, time::{Duration, Instant}
};

use freetype::{self, face::LoadFlag, Face, GlyphMetrics, GlyphSlot, Library};
use image::{GrayImage, Luma};
use nalgebra_glm::{Vec2, Vec3};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib}, camera::{Camera, CameraProjection, CameraView}, shader_builder::{
        shader::{CompileProgram, CompileProgramFromString},
        ColorAssignment, ShaderAttribute, ShaderAttributeType, ShaderBuilder,
    }, shaders, texts::TextGenerator, textures::Texture, window::Window, FONT_PATH
};


use glfw::Context;
fn main() {
    let mut window = Window::default();

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunci(0, gl::ONE, gl::ONE);
        gl::BlendFunci(1, gl::ONE, gl::ONE_MINUS_SRC_COLOR);
    }

    let ft_lib = Library::init().unwrap();
    let face = ft_lib.new_face(FONT_PATH, 0).unwrap();

    let text_gen = TextGenerator::new(&face, 100);

    text_gen.generate_text_texture("Hello World124 f(x)").0.save("hw.png").unwrap();

    // return ;
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
        atlas.get_atlas_image().width() as i32,
        atlas.get_atlas_image().height() as i32,
        atlas.get_atlas_image().as_ptr() as _,
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

            gl::DrawArrays(gl::TRIANGLES, 0, text.len() as _);
        }

        window.m_window.swap_buffers();
    }
}

fn main2() {
    let freetype_library = freetype::library::Library::init().unwrap();
    let face = freetype_library.new_face(FONT_PATH, 0).unwrap();

    let chars = TextGenerator::new(&face, 100);
    let mut char_counter = 0;
    let a = Instant::now();
    // println!("{:?}", chars.generate_text("Hello World"));

    for i in 0..100 {
        let t = i as f32 / 99.0;
        let tmp = chars.generate_text_texture(format!("f({t})=sin=sin=sin=sin({t})").as_str());
        char_counter += tmp.1;
    }

    let atlas = chars.generate_normalized_atlas();
    println!("{:?}", atlas.get_metrics_vector()[64]);

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
