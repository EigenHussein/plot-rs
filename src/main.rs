use glfw::{Action, Context, WindowEvent};
use nalgebra_glm::Vec3;
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    shader_builder::{self, RawShader, ShaderBuilder},
    window::Window,
};
use std::mem::MaybeUninit;
use std::mem::{size_of, size_of_val};

fn main() {
    //   println!("{}", unsafe{ len(&Vec::<(u32,f64)>::with_capacity(100).len())});

    // return;

    let mut window = Window::default();

    let mut shaderbuilder = ShaderBuilder::default();
    shaderbuilder.set_color_assignment(shader_builder::ColorAssignment::PerPosition);
    println!(
        "{}\n\n\n{}",
        shaderbuilder.generate_vertex_shader(),
        shaderbuilder.generate_fragment_shader()
    );
    let shader = shaderbuilder.compile_shader();

    // let vertices = [
    //     Vec3::new(-1.0, -1.0, 0.0),
    //     Vec3::new(1.0, -1.0, 0.0),
    //     Vec3::new(1.0, 1.0, 0.0),
    //     Vec3::new(-1.0, -1.0, 0.0),
    //     Vec3::new(1.0, 1.0, 0.0),
    //     Vec3::new(-1.0, 1.0, 0.0),
    // ];
    // let buffer = ArrayBuffer::default_buffer_from_slice(
    //     vertices.as_slice(),
    //     None,
    //     &[GLAttrib::new(0, 3, 3, 0)].to_vec(),
    //     true,
    // );

    let vertices = [
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(0.5, -0.5, 0.0),
        Vec3::new(0.5, 0.5, 0.0),
        Vec3::new(-0.5, 0.5, 0.0),
    ];

    let indices = [0u32, 1u32, 2u32, 0u32, 2u32, 3u32];

    let buffer = ArrayBuffer::default_buffer_from_slice(
        vertices.as_slice(),
        Some(&indices),
        &[GLAttrib::new(0, 3, 3, 0)].to_vec(),
        true,
    );

    buffer.ebo.unbind();

    let mut cam = Camera::default();
//cam.set_camera(&Vec3::new(1.0, 1.0, -1.0), &Vec3::zeros(), &Vec3::z());
    cam.add_program(shader.get_id()).update_uniforms();

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            update_camera(&event, &mut cam);
        }

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(shader.get_id());
            buffer.vao.bind();
            buffer.ebo.bind();
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
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::D => {
                        camera_instance
                            .get_camera_view()
                            .decrease_azimuth(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::W => {
                        camera_instance
                            .get_camera_view()
                            .increase_altitude(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::S => {
                        camera_instance
                            .get_camera_view()
                            .decrease_altitude(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }

                    _ => {}
                };
            }
        }
        _ => {}
    };
}
