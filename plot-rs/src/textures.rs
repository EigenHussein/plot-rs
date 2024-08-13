use gl::types::GLint;
use nalgebra_glm::{Mat4, Vec2, Vec4};

use crate::{
    buffers::GLAttrib,
    camera::{Camera, CameraProjection, CameraView}, shader_builder::RawShader,
};

pub struct Texture {
    texture_id: u32,
    width: i32,
    height: i32,
}
impl Texture {
    /// Sets the default texture parameters for 2D textures.
    ///
    /// This function sets the following parameters:
    /// - `gl::TEXTURE_MIN_FILTER` to `gl::LINEAR`, which specifies linear filtering for minimizing.
    /// - `gl::TEXTURE_MAG_FILTER` to `gl::LINEAR`, which specifies linear filtering for magnifying.
    /// - `gl::TEXTURE_WRAP_S` to `gl::REPEAT`, which repeats the texture horizontally.
    /// - `gl::TEXTURE_WRAP_T` to `gl::REPEAT`, which repeats the texture vertically.
    ///
    /// Note: The trilinear filtering option (`gl::LINEAR_MIPMAP_LINEAR`) is commented out.
    /// To enable trilinear filtering, uncomment the relevant line.
    pub fn texture_set_default_parameters() {
        unsafe {
            // gl::TexParameteri(
            //     gl::TEXTURE_2D,
            //     gl::TEXTURE_MIN_FILTER,
            //     gl::LINEAR_MIPMAP_LINEAR as _,
            // ); // Trilinear filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
        }
    }

    /// Creates a new texture with the specified parameters.
    ///
    /// This function generates a new 2D texture with the given width and height.
    /// The texture's internal format, external format, data type, and initial data can be optionally specified.
    /// If these optional parameters are not provided, default values are used:
    /// - `internal` defaults to `gl::RGBA16F`
    /// - `external_format` defaults to `gl::RGBA`
    /// - `data_type` defaults to `gl::UNSIGNED_BYTE`
    /// - `data_ptr` defaults to a null pointer, indicating no initial data
    ///
    /// The function performs the following steps:
    /// 1. Generates a new texture ID.
    /// 2. Binds the texture as a 2D texture.
    /// 3. Specifies the texture image with the given or default parameters.
    /// 4. Sets the default texture parameters using `texture_set_default_parameters()`.
    /// 5. (Optionally) unbinds the texture (currently commented out).
    ///
    /// # Parameters
    /// - `width`: The width of the texture.
    /// - `height`: The height of the texture.
    /// - `internal`: Optional internal format of the texture.
    /// - `external_format`: Optional external format of the texture.
    /// - `data_type`: Optional data type of the texture.
    /// - `data_ptr`: Optional pointer to the initial data for the texture.
    ///
    /// # Returns
    /// A new instance of the texture, with the texture ID and specified dimensions.
    pub fn new(
        width: i32,
        height: i32,
        internal: Option<u32>,
        external_format: Option<u32>,
        data_type: Option<u32>,
        data_ptr: Option<*const u8>,
    ) -> Self {
        let mut texture = 0u32;
        unsafe {
            gl::GenTextures(1, &mut texture as _);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal.unwrap_or(gl::RGBA16F) as _,
                width,
                height,
                0,
                external_format.unwrap_or(gl::RGBA) as _,
                data_type.unwrap_or(gl::UNSIGNED_BYTE),
                data_ptr.unwrap_or(0 as _) as _,
            );
            Self::texture_set_default_parameters();
            // gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self {
            texture_id: texture,
            width: width,
            height: height,
        }
    }
    /// Creates a new RGBA texture with the specified width, height, and data.
    ///
    /// This function is a convenience wrapper for creating a texture with:
    /// - `internal` format: `gl::RGBA8UI`
    /// - `external_format`: `gl::RGBA`
    /// - `data_type`: default (None)
    /// - `data_ptr`: specified data
    ///
    /// # Parameters
    /// - `width`: The width of the texture.
    /// - `height`: The height of the texture.
    /// - `data`: Pointer to the initial RGBA data.
    ///
    /// # Returns
    /// A new instance of the texture.
    pub fn new_rgba(width: i32, height: i32, data: *const u8) -> Self {
        Self::new(
            width,
            height,
            Some(gl::RGBA8),
            Some(gl::RGBA),
            None,
            Some(data),
        )
    }

    /// Creates a new monochrome texture with the specified width, height, and data.
    ///
    /// This function sets the pixel unpack alignment to 1 and creates a texture with:
    /// - `internal` format: `gl::R32F`
    /// - `external_format`: `gl::RED`
    /// - `data_type`: default (None)
    /// - `data_ptr`: specified data
    ///
    /// # Parameters
    /// - `width`: The width of the texture.
    /// - `height`: The height of the texture.
    /// - `data`: Pointer to the initial monochrome data.
    ///
    /// # Returns
    /// A new instance of the texture.
    pub fn new_mono(width: i32, height: i32, data: *const u8) -> Self {
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }
        Self::new(
            width,
            height,
            Some(gl::R32F),
            Some(gl::RED),
            None,
            Some(data),
        )
    }
    /// Returns the texture's ID, width, and height as a tuple.
    ///
    /// # Returns
    /// A tuple containing:
    /// - `texture_id`: The texture ID.
    /// - `width`: The width of the texture.
    /// - `height`: The height of the texture.
    pub fn get_all(&self) -> (u32, i32, i32) {
        return (self.texture_id, self.width, self.height);
    }
    /// Sets the active texture unit and returns a reference to self.
    ///
    /// This function sets the active texture unit to the specified texture number.
    ///
    /// # Parameters
    /// - `texture_number`: The texture unit number to activate (e.g., 0 for `gl::TEXTURE0`).
    ///
    /// # Returns
    /// A reference to the current texture instance.
    pub fn set_active_texture(&self, texture_number: u32) -> &Self {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_number);
        }
        self
    }
    /// Binds the texture and returns a reference to self.
    ///
    /// This function binds the current texture to `gl::TEXTURE_2D`.
    ///
    /// # Returns
    /// A reference to the current texture instance.
    pub fn bind_texture(&self) -> &Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
        self
    }
    /// Unbinds the texture and returns a reference to self.
    ///
    /// This function unbinds the current texture from `gl::TEXTURE_2D`.
    ///
    /// # Returns
    /// A reference to the current texture instance.
    pub fn unbind_texture(&self) -> &Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        self
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        println!("Deleting texture!");
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

pub struct FrameBuffer {
    pub fbo: u32,
    pub rbo_depth: u32,
    pub rbo_color: u32,
    pub fbo_texture: u32,
    pub destination_texture: u32,
    pub width: i32,
    pub height: i32,
}

impl FrameBuffer {
    pub fn texture_set_default_parameters() {
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as _,
            ); // Trilinear filtering
               //    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR  as _);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR_MIPMAP_LINEAR as _);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
        }
    }

    pub fn texture_generate(
        width: i32,
        height: i32,
        internal: Option<u32>,
        external_format: Option<u32>,
    ) -> u32 {
        let mut texture = 0u32;
        unsafe {
            gl::GenTextures(1, &mut texture as _);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal.unwrap_or(gl::RGBA16F) as _,
                width,
                height,
                0,
                external_format.unwrap_or(gl::RGBA) as _,
                gl::FLOAT,
                0 as _,
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
        }
        texture
    }

    pub fn framebuffer_generate() -> u32 {
        unsafe {
            let mut fbo = 0u32;
            gl::GenFramebuffers(1, &mut fbo as _);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            return fbo;
        }
    }

    pub fn renderbuffer_generate(width: i32, height: i32, multisample: bool) -> (u32, u32) {
        let mut depthBuffer = 0u32;
        let mut colorBuffer = 0u32;
        unsafe {
            gl::GenRenderbuffers(1, &mut depthBuffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depthBuffer);
            gl::RenderbufferStorageMultisample(
                gl::RENDERBUFFER,
                16,
                gl::DEPTH24_STENCIL8,
                width,
                height,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                depthBuffer,
            );

            // Create multisampled color attachment

            gl::GenRenderbuffers(1, &mut colorBuffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, colorBuffer);
            gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 16, gl::RGBA8, width, height);
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::RENDERBUFFER,
                colorBuffer,
            );
        }
        (depthBuffer, colorBuffer)
    }

    pub fn new(width: i32, height: i32) -> Self {
        let mut fbo = 0u32;
        let (mut rbo_depth, mut rbo_color) = (0u32, 0u32);
        let mut texture = 0u32;
        unsafe {
            fbo = Self::framebuffer_generate();
            println!("Generated FBO");
            texture = Self::texture_generate(width, height, None, None);
            Self::texture_set_default_parameters();
            // Generate and bind RBO with multisampling
            (rbo_depth, rbo_color) = Self::renderbuffer_generate(width, height, true);

            // gl::GenTextures(1, &mut texture);
            println!("Generated Render Buffer");

            // gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);

            // Check framebuffer completeness
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("Framebuffer is not complete");
            } else {
                println!("FrameBuffer: Success");
            }

            // Unbind framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Self {
            fbo: fbo,
            rbo_depth: rbo_depth,
            rbo_color: rbo_color,
            destination_texture: 0,
            fbo_texture: texture,
            height: height,
            width: width,
        }
    }

    pub fn new2(width: i32, height: i32) -> Self {
        unsafe {
            let mut fbo = 0;
            let mut rbo = 0;
            let mut tex = 0;

            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            //gl::RenderbufferStorage(gl::RENDERBUFFER, gl::RGBA16F, width, height);
            gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 16, gl::RGBA16F, width, height);

            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::RENDERBUFFER,
                rbo,
            );

            let mut rbo_depth = 0;
            gl::GenRenderbuffers(1, &mut rbo_depth);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_depth);
            //gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
            gl::RenderbufferStorageMultisample(
                gl::RENDERBUFFER,
                16,
                gl::DEPTH_COMPONENT,
                width,
                height,
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

            gl::GenTextures(1, &mut tex);

            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA16F as _,
                width,
                height,
                0,
                gl::RGBA as _,
                gl::FLOAT,
                0 as _,
            );
            FrameBuffer::texture_set_default_parameters();

            let mut fbo_tex = 0;
            gl::GenFramebuffers(1, &mut fbo_tex);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_tex);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex,
                0,
            );
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer not complete!");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            Self {
                fbo: fbo,
                fbo_texture: fbo_tex,
                rbo_depth: rbo_depth,
                rbo_color: rbo,
                destination_texture: tex,
                width: width,
                height: height,
            }
        }
    }

    pub fn bint_to_texture(&self) -> u32 {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo_texture);
            gl::BlitFramebuffer(
                0,
                0,
                self.width,
                self.height,
                0,
                0,
                self.width,
                self.height,
                gl::COLOR_BUFFER_BIT,
                gl::LINEAR,
            );
        }
        self.destination_texture
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}



pub struct VirtualScreen {
    pub shader: u32,
    camera: Camera,
    pub frame_buffer: FrameBuffer,
    pub simple_fbo: u32,
    pub simple_tx: u32,
}

impl VirtualScreen {
    pub fn get_quad() -> [(Vec4, Vec4, Vec2); 4] {
        let quad_vertices = [
            (
                Vec4::new(-1.0, -1.0, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec2::new(0.0, 0.0),
            ),
            (
                Vec4::new(-1.0, 1.0, 0.0, 1.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
                Vec2::new(0.0, 1.0),
            ),
            (
                Vec4::new(1.0, 1.0, 0.0, 1.0),
                Vec4::new(0.0, 1.0, 0.0, 1.0),
                Vec2::new(1.0, 1.0),
            ),
            (
                Vec4::new(1.0, -1.0, 0.0, 1.0),
                Vec4::new(0.0, 0.0, 1.0, 1.0),
                Vec2::new(1.0, 0.0),
            ),
        ];
        quad_vertices
    }
    // pub fn new(width: i32, height: i32) -> Self {
    //     let quad_vertices = Self::get_quad();
    //     let indices = [0, 1, 2, 0, 2, 3];

    //     let mut texture_prog =
    //         BasicShader::new(ShaderCreator::new(shaders::ColorMode::ColorBloom, false).program_id);
    //     let mut cam = Camera::new(
    //         CameraProjection::default_perspective(1.0, 1.5),
    //         CameraView::default(),
    //     );

    //     texture_prog.buffer.allocateBuffer(
    //         quad_vertices.as_ptr() as _,
    //         std::mem::size_of_val(&quad_vertices) as _,
    //     );
    //     println!(
    //         "sizes: {}\t{}",
    //         std::mem::size_of_val(&indices),
    //         std::mem::size_of_val(&quad_vertices)
    //     );
    //     texture_prog
    //         .buffer
    //         .allocateElements(indices.as_ptr() as _, std::mem::size_of_val(&indices) as _);
    //     texture_prog.buffer.setAttributes(&vec![
    //         GLAttrib::new(0, 4, 10, 0),
    //         GLAttrib::new(1, 4, 10, 4),
    //         GLAttrib::new(3, 2, 10, 8),
    //     ]);

    //     cam.add_program(texture_prog.get_program());
    //     cam.set_projection_matrix(Mat4::identity());
    //     cam.set_view_matrix(Mat4::identity());

    //     let virtualframe = FrameBuffer::new(width, height);

    //     cam.update_uniforms(None);

    //     let (mut s_fbo, mut s_tex) = (0u32, 0u32);
    //     unsafe {
    //         gl::GenFramebuffers(1, &mut s_fbo);
    //         gl::BindFramebuffer(gl::FRAMEBUFFER, s_fbo);
    //         gl::GenTextures(1, &mut s_tex);
    //         gl::BindTexture(gl::TEXTURE_2D, s_tex);
    //         gl::TexImage2D(
    //             gl::TEXTURE_2D,
    //             0,
    //             gl::RGBA16F as _,
    //             width,
    //             height,
    //             0,
    //             gl::RGBA,
    //             gl::FLOAT,
    //             0 as _,
    //         );
    //         FrameBuffer::texture_set_default_parameters();
    //         gl::FramebufferTexture2D(
    //             gl::FRAMEBUFFER,
    //             gl::COLOR_ATTACHMENT0,
    //             gl::TEXTURE_2D,
    //             s_tex,
    //             0,
    //         );
    //         gl::BindTexture(gl::TEXTURE_2D, 0);
    //         gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    //     }

    //     Self {
    //         shader: texture_prog,
    //         camera: cam,
    //         frame_buffer: virtualframe,
    //         simple_fbo: s_fbo,
    //         simple_tx: s_tex,
    //     }
    // }
    // pub fn render_new_to_frame_buffer<T>(&self, mut render_call: T)
    // where
    //     T: FnMut() -> (),
    // {
    //     unsafe {
    //         gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer.fbo);
    //         // gl::Viewport(0, 0, self.frame_buffer.width, self.frame_buffer.height);
    //         // gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    //         // gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    //         // gl::UseProgram(self.shader.program);
    //         render_call();
    //         gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

    //         gl::BlitNamedFramebuffer(
    //             self.frame_buffer.fbo,
    //             self.simple_fbo,
    //             0,
    //             0,
    //             1024,
    //             1024,
    //             0,
    //             0,
    //             1024,
    //             1024,
    //             gl::COLOR_BUFFER_BIT,
    //             gl::LINEAR,
    //         );

    //         gl::ActiveTexture(gl::TEXTURE0);
    //         gl::BindTexture(gl::TEXTURE_2D, self.simple_tx);
    //         gl::GenerateMipmap(gl::TEXTURE_2D);

    //         gl::ClearColor(0.01, 0.01, 0.01, 1.0);
    //         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    //         gl::UseProgram(self.shader.program);
    //         self.shader.buffer.bindAB();
    //         gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
    //         // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
    //     }
    // }
}
