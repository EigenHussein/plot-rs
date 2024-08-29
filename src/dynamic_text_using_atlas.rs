use glfw::Context;
use nalgebra_glm::{pi, vec3_to_vec4, Vec2, Vec3, Vec4};
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::{Camera, CameraProjection, CameraView},
    coordinate_mapping::Coordinate,
    shader_builder::{shader::CompileProgram, RawShader},
    texts::{DynamicTexts, FreeTypeAtlas, TextGenerator},
    textures::Texture,
    window::Window,
};

fn main() {
    let mut window = Window::new(Some("Cycloid"), Some(1600), Some(900)).unwrap();

    let mut cam = Camera::new(
        CameraProjection::default_perspective(16.0 / 9.0, 1.3),
        CameraView::default(),
    );

    // let lib = freetype::Library::init().unwrap();
    // let face = lib.new_face("cambriaz.ttf", 0).unwrap();

    // let mut text_generator = TextGenerator::new(&face, 100);
    // let atlas = text_generator.generate_normalized_atlas();

    let mut tboxes = DynamicTexts::new("cambriaz.ttf", 100, 300).unwrap();
    tboxes
        .add_text("Hello World!", 1.0, &Coordinate::default())
        .add_text(
            "Y Axis",
            1.0,
            &Coordinate::default()
                .rotate_ccw_w(std::f32::consts::PI / 2.00).translate(Vec3::new(0.0,1.0,0.0))
                .clone(),
        )
        .update_buffer()
        .bind_camera(&mut cam);

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            Camera::update_camera(&event, &mut cam);
        }

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            tboxes.draw_on();
        }

        window.m_window.swap_buffers();
    }
}
