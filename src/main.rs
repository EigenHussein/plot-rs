use plot_rs::{
    shaders::{self, RawShader, ShaderBuilder},
    window::Window,
};
use glfw::Context;

fn main() {
    let mut window = Window::default();

    let mut shaderbuilder = ShaderBuilder::default();
    println!(
        "{}\n\n\n{}",
        shaderbuilder.generate_vertex_shader(),
        shaderbuilder.generate_fragment_shader()
    );
    let shader = shaderbuilder.compile_shader();

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        window.m_window.swap_buffers();
    }
}
