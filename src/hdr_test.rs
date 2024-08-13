use std::ptr;

use gl::types::{GLint, GLsizei, GLuint};
use glfw::Context;

use nalgebra_glm::Vec3;
use plot_rs::{
    buffers::{ArrayBuffer, GLAttrib},
    camera::Camera,
    shader_builder::{shader::CompileProgram, ShaderAttribute, ShaderBuilder},
    shapes::TwoDim,
    textures::{FrameBuffer, VirtualScreen},
    window::Window,
};

fn get_program() -> plot_rs::shader_builder::RawShader {
    let mut builder = ShaderBuilder::empty();
    builder
        .set_vertex_shader_input_attributes(Some(
            ShaderAttribute::generate_default_vertex_input_attributes().to_vec(),
        ))
        // .set_vertex_transformation(plot_rs::shader_builder::VertexTransformation::ProjectionView)
        .add_vertex_uniform(ShaderAttribute::generate_uniform(
            None,
            "Proj".to_string(),
            plot_rs::shader_builder::ShaderAttributeType::Mat4,
        ))
        .add_vertex_uniform(ShaderAttribute::generate_uniform(
            None,
            "View".to_string(),
            plot_rs::shader_builder::ShaderAttributeType::Mat4,
        ))
        .set_vertex_transformation(plot_rs::shader_builder::VertexTransformation::Custom(
            "gl_Position = Proj*View*vec4(position.xyz,1);".to_string(),
        ))
        .add_vertex2fragment_link(
            "color".to_string(),
            ShaderAttribute::new_output_attribute(
                None,
                "v_col".to_string(),
                plot_rs::shader_builder::ShaderAttributeType::Vec3,
            ),
        )
        .set_color_assignment(plot_rs::shader_builder::ColorAssignment::Custom(
            r"
             Color = vec4(v_col/2,1);
     
        
            "
            .to_string(),
        ));

    println!(
        "{},{}",
        builder.generate_vertex_shader(),
        builder.generate_fragment_shader()
    );
    builder.compile_shader()
}
unsafe fn create_hdr_framebuffer(
    screen_width: u32,
    screen_height: u32,
) -> (GLuint, GLuint, GLuint) {
    let mut hdr_fbo: GLuint = 0;
    gl::GenFramebuffers(1, &mut hdr_fbo);
    gl::BindFramebuffer(gl::FRAMEBUFFER, hdr_fbo);

    // Create a floating-point color buffer
    let mut color_buffer: GLuint = 0;
    gl::GenTextures(1, &mut color_buffer);
    gl::BindTexture(gl::TEXTURE_2D, color_buffer);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA16F as GLint,
        screen_width as GLsizei,
        screen_height as GLsizei,
        0,
        gl::RGBA,
        gl::FLOAT,
        ptr::null(),
    );
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_S,
        gl::CLAMP_TO_EDGE as GLint,
    );
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_WRAP_T,
        gl::CLAMP_TO_EDGE as GLint,
    );
    gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::COLOR_ATTACHMENT0,
        gl::TEXTURE_2D,
        color_buffer,
        0,
    );

    // Create and attach a depth buffer (renderbuffer)
    let mut rbo_depth: GLuint = 0;
    gl::GenRenderbuffers(1, &mut rbo_depth);
    gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_depth);
    gl::RenderbufferStorage(
        gl::RENDERBUFFER,
        gl::DEPTH_COMPONENT,
        screen_width as GLsizei,
        screen_height as GLsizei,
    );
    gl::FramebufferRenderbuffer(
        gl::FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        gl::RENDERBUFFER,
        rbo_depth,
    );

    if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
        panic!("Framebuffer not complete!");
    }

    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

    (hdr_fbo, color_buffer, rbo_depth)
}

fn test(width: i32, height: i32) {
    unsafe {
        let mut fbo = 0;
        let mut tex = 0;
        let mut rbo_depth = 0;
        let mut rbo_msaa = 0;
        let samples = 16; // Number of samples for multisampling

        // Generate and bind the framebuffer
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        // Generate and configure the texture
        // gl::GenTextures(1, &mut tex);
        // gl::BindTexture(gl::TEXTURE_2D, tex);
        // gl::TexImage2D(
        //     gl::TEXTURE_2D,
        //     0,
        //     gl::RGBA16F as _,
        //     width,
        //     height,
        //     0,
        //     gl::RGBA,
        //     gl::FLOAT,
        //     std::ptr::null(),
        // );
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
        // gl::FramebufferTexture2D(
        //     gl::FRAMEBUFFER,
        //     gl::COLOR_ATTACHMENT0,
        //     gl::TEXTURE_2D,
        //     tex,
        //     0,
        // );

        // Generate and configure the multisampled renderbuffer for color attachment
        gl::GenRenderbuffers(1, &mut rbo_msaa);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_msaa);
        gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, samples, gl::RGBA16F, width, height);
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::RENDERBUFFER,
            rbo_msaa,
        );

        // Generate and configure the renderbuffer for depth attachment
        gl::GenRenderbuffers(1, &mut rbo_depth);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_depth);
        gl::RenderbufferStorageMultisample(
            gl::RENDERBUFFER,
            samples,
            gl::DEPTH_COMPONENT24,
            width,
            height,
        );
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo_depth,
        );

        // Check if the framebuffer is complete
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("Framebuffer not complete!");
        }

        // Unbind the framebuffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}
fn main() {
    let mut window = Window::default();
    // test(1024, 1024);
    let line = TwoDim::Quad::new_from_2points(&Vec3::zeros(), &Vec3::x(), 0.20, Some(&Vec3::z()));
    let colors = [
        1.0*Vec3::new(1.0, 1.0, 1.05),
        1.0*Vec3::new(1.0, 1.0, 1.00),
        1.0*Vec3::new(1.0, 1.0, 1.0),
        1.0*Vec3::new(1.0, 1.0, 1.0),
        1.0*Vec3::new(1.0, 1.0, 1.0),
        1.0*Vec3::new(1.0, 1.0, 1.0),
    ];
    let colored_line: Vec<(Vec3, Vec3)> = (0..6).map(|i| (line[i], colors[i])).collect();
    let mut ab = ArrayBuffer::default_buffer_from_slice(
        &colored_line,
        None,
        &[GLAttrib::new(0, 3, 6, 0), GLAttrib::new(2, 3, 6, 3)].to_vec(),
        true,
    );

    // let mut program = CompileProgram(
    //     "/home/hussein/Dev/Rust/plotters/shaders/shapes/vertex.glsl",
    //     "/home/hussein/Dev/Rust/plotters/shaders/shapes/fragment.glsl",
    //     None,
    // );

    let prog = get_program();
    let program = prog.get_id();

    let mut cam = Camera::default();
    cam.set_perspective(1.0, 1.5, 0.01, 100.0)
        .add_program(program)
        .update_uniforms(None);

    let mut fb = FrameBuffer::new2(1024, 1024);

    fb.bind();

    // Building texture program

    let tx_prog = CompileProgram(
        "shaders/hdr/bloom_vertex.glsl",
        "shaders/hdr/bloom_fragment.glsl",
        None,
    );

    let mut texture_array_buffer = ArrayBuffer::default_buffer_from_slice(
        &VirtualScreen::get_quad(),
        Some(&[0, 1, 2, 0, 2, 3]),
        &[
            GLAttrib::new(0, 4, 10, 0),
            GLAttrib::new(1, 4, 10, 4),
            GLAttrib::new(2, 2, 10, 8),
        ]
        .to_vec(),
        true,
    );

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            Camera::update_camera(&event, &mut cam);
        }

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb.fbo);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);
            ab.vao.bind();
            ab.vbo.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            fb.bint_to_texture();

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::ClearColor(0.0, 0.0, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::BindTexture(gl::TEXTURE_2D, fb.destination_texture);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::UseProgram(tx_prog);
            texture_array_buffer.vao.bind();
            texture_array_buffer.vbo.bind();
            texture_array_buffer.ebo.bind();

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
        }
        window.m_window.swap_buffers();
    }
}
