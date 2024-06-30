use std::{
    os::raw::c_void,
    sync::{Arc, Mutex},
};

use nalgebra_glm::Vec4;

use crate::{
    buffers::ArrayBuffer,
    camera::{Camera, CameraProjection, CameraView},
    shader_builder::RawShader,
};

pub enum DrawCall {
    // unsafe { gl::DrawArrays(*mode, *first as i32, *count as i32) };
    Vertex(u32, i32, i32),
    // unsafe { gl::DrawElements(*mode, *count, *type_, *indices) };
    Element(u32, i32, u32, *const c_void),
    // Special
    Custom(Box<dyn Fn()->()>)
}

impl DrawCall {
    pub fn draw(&self) {
        match self {
            DrawCall::Vertex(mode, first, count) => {
                unsafe { gl::DrawArrays(*mode, *first as i32, *count as i32) };
            }
            DrawCall::Element(mode, count, type_, indices) => {
                unsafe { gl::DrawElements(*mode, *count, *type_, *indices) };
            }
            DrawCall::Custom(custom_call)=>{
                custom_call();
            }
        }
    }
}

pub struct Program {
    pub shader: RawShader,
    pub buffer: ArrayBuffer,
    pub drawcall: DrawCall,
}
pub trait SceneLayer {
    fn get_program_id(&self) -> u32;
    fn get_layer(&self) -> Arc<Mutex<Program>>;
    fn bind_camera(&self, camera: &mut Camera);
    fn draw(&self);
}

pub struct Scene {
    camera: Camera,
    layers: Vec<Arc<Mutex<Program>>>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            camera: Camera::new(
                CameraProjection::default_perspective(1.0, 1.8),
                CameraView::default(),
            ),
            layers: Vec::new(),
        }
    }
}

impl Scene {
    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub fn add_layer(&mut self, layer: Arc<Mutex<Program>>) {
        self.camera
            .add_program(layer.lock().unwrap().shader.get_id());
        self.layers.push(layer);
    }
    pub fn draw_on(&self) -> &Self {
        for layer in self.layers.iter() {
            let tmp_layer = layer.lock().unwrap();
            tmp_layer.buffer.vao.bind();
            tmp_layer.shader.use_shader();
            tmp_layer.drawcall.draw();
        }
        self
    }
    pub fn draw(&self, background_color: Option<Vec4>) -> &Self {
        let background_color = if let Some(bg) = background_color {
            bg
        } else {
            Vec4::new(0.0, 0.0, 0.0, 1.0)
        };
        unsafe {
            gl::ClearColor(
                background_color.x,
                background_color.y,
                background_color.z,
                background_color.w,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.draw_on();
        self
    }
}
