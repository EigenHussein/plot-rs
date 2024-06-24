use nalgebra_glm::Vec4;

use crate::camera::{Camera, CameraProjection, CameraView};

pub trait SceneLayer {
    fn get_program_id(&self) -> u32;
    fn bind_camera(&self, camera: &mut Camera);
    fn draw(&self);
}

pub struct Scene {
    camera: Camera,
    layers: Vec<Box<dyn SceneLayer>>,
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
    pub fn add_layer(&mut self, layer: Box<dyn SceneLayer>) {
        self.camera.add_program(layer.get_program_id());
        self.layers.push(layer);
        
    }
    pub fn draw_on(&self) -> &Self {
        for layer in self.layers.iter() {
            layer.draw();
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
