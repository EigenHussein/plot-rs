use glfw::{Action, Context, WindowEvent};
use nalgebra_glm::Vec3;
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib}, camera::{Camera, CameraProjection, CameraView}, layers::{Drawable, MathLayers::LineLayer}, scene::Scene, shader_builder::{self, RawShader, ShaderBuilder}, window::Window
};

use std::mem::{size_of, size_of_val};

fn main(){
    let mut window = Window::default();

    let mut my_function = LineLayer::simple_colored_line(
        |t| Vec3::new(f32::sin(1.0 * t), f32::sin(1.0 * t), f32::sin(2.0 * t)),
        0.0,
        5.0,
        4000,
        None,
        |t| {
            Vec3::new(0.5, 0.5, 0.5)
                + 0.5 * Vec3::new(f32::sin(10.0 * t), f32::sin(15.0 * t), f32::sin(24.0 * t))
        },
        false,
    );

    let mut my_scene = Scene::default();
    my_scene.add_layer(Box::new(my_function));

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            update_camera(&event, my_scene.get_camera());
        }
        my_scene.draw(None);
        window.m_window.swap_buffers();

    }

}

fn main3() {

    let mut window = Window::default();

    let mut shaderbuilder = ShaderBuilder::default();
    shaderbuilder.set_color_assignment(shader_builder::ColorAssignment::PerPosition);
    println!(
        "{}\n\n\n{}",
        shaderbuilder.generate_vertex_shader(),
        shaderbuilder.generate_fragment_shader()
    );
    let shader = shaderbuilder.compile_shader();

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
        false,
    );

    buffer.ebo.unbind();

    let mut my_function = LineLayer::simple_colored_line(
        |t| Vec3::new(f32::sin(1.0 * t), f32::sin(1.0 * t), f32::sin(2.0 * t)),
        0.0,
        5.0,
        4000,
        None,
        |t| {
            Vec3::new(0.5, 0.5, 0.5)
                + 0.5 * Vec3::new(f32::sin(10.0 * t), f32::sin(15.0 * t), f32::sin(24.0 * t))
        },
        false,
    );

    let mut cam = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );
    cam.set_camera(&Vec3::new(1.0, 1.0, -1.0), &Vec3::zeros(), &Vec3::z())
        .add_program(shader.get_id())
        .add_program(my_function.shader.get_id());
        // .update_uniforms(None);

    let mut z = 0.0f32;

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
            my_function.update_line_function(
                |t| {
                    Vec3::new(
                        f32::sin(1.0 * z * t),
                        f32::sin(3.0 * z * t),
                        f32::sin(3.1430 * z * t),
                    )
                },
                0.0,
                5.0,
            );
            my_function.draw_on();
        }
        z += 0.01;


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
