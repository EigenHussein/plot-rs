use std::any::Any;

use freetype::{Face, Library};
use glfw::{Action, Context, WindowEvent};
use nalgebra_glm::{Vec2, Vec4};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    shader_builder::{ColorAssignment, ShaderAttribute, ShaderAttributeType, ShaderBuilder},
    texts::FreeTypeGenerator,
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
fn main() {
    let mut window = Window::default();

    let text_gen = FreeTypeGenerator::new(Library::init().unwrap(), None);
    let img = text_gen.generate_image("Hello World");

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
        .set_vertex_transformation(plot_rs::shader_builder::VertexTransformation::Custom("gl_Position=Proj*View*vec4(position.xyz,2);".to_string()))
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
    // let texture = Texture::new_mono(
    //     img.width() as _,
    //     img.height() as _,
    //     img.as_ptr() as _,
    // );
    // texture.bind_texture().set_active_texture(0);

    unsafe {
        gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        Texture::texture_set_default_parameters();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::R32F as _,
            img.width() as _,
            img.height() as _,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            img.as_ptr() as _,
        );
        // gl::ActiveTexture(gl::TEXTURE0);
        // gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    let mut cam = Camera::default();
    cam.add_program(shade.get_id()).update_uniforms(None);
    ab.vao.bind();
    ab.ebo.bind();
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
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
