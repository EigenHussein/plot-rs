use std::{
    collections::HashMap,
    ffi::{c_void, CString},
};

use glfw::{Action, WindowEvent};
use nalgebra_glm::{Mat4, Vec3, Vec4};

use crate::shader_builder::ShaderUniforms;

pub struct Camera {
    programs: Vec<ShaderUniforms>,
    uniforms_names: [CString; 3],
    m_projection: nalgebra_glm::Mat4, //Option<glm::Mat4>,
    // CAMERA BEGIN
    m_view: CameraView,
    // CAMERA END
    m_model: nalgebra_glm::Mat4,
}

impl<'a> Default for Camera {
    fn default() -> Self {
        Self {
            programs: Vec::new(),
            uniforms_names: [
                CString::new("Proj").unwrap(),
                CString::new("View").unwrap(),
                CString::new("Model").unwrap(),
            ],
            m_projection: nalgebra_glm::Mat4::identity(),
            m_view: *CameraView::default()
                .set_center(&Vec3::zeros())
                .set_eye(&Vec3::new(1.0, 1.0, 1.0))
                .set_up(&Vec3::z()),
            // CameraView {
            //     view_matrix: nalgebra_glm::Mat4::identity(),
            //     ..Default::default()
            // },
            m_model: nalgebra_glm::Mat4::identity(),
        }
    }
}

impl<'a> Camera {
    /// ### Description
    /// * Getting scene matrices ready
    /// #### Development Requirements
    /// * Add model matrix update mechanism
    ///
    pub fn new(projection: CameraProjection, view: CameraView) -> Self {
        Camera {
            programs: Vec::new(),
            uniforms_names: [
                CString::new("Proj").unwrap(),
                CString::new("View").unwrap(),
                CString::new("Model").unwrap(),
            ],
            m_projection: projection.projection_matrix,
            m_view: view,
            m_model: nalgebra_glm::Mat4::identity(),
        }
    }

    pub fn set_perspective(&mut self, aspect: f32, fovy: f32, near: f32, far: f32) -> &mut Self {
        self.m_projection = nalgebra_glm::perspective(aspect, fovy, near, far);
        self
    }
    pub fn set_ortho(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        znear: f32,
        zfar: f32,
    ) -> &mut Self {
        self.m_projection = nalgebra_glm::ortho(left, right, bottom, top, znear, zfar);
        self
    }
    pub fn set_camera(&mut self, eye: &Vec3, center: &Vec3, up: &Vec3) -> &mut Self {
        self.m_view = CameraView {
            eye: *eye,
            center: *center,
            up: *up,
            ..Default::default()
        };
        self.m_view.update_matrix();
        self
    }
    pub fn get_camera_view(&mut self) -> &mut CameraView {
        &mut self.m_view
    }

    pub fn set_projection_matrix(&mut self, matrix: Mat4) -> &mut Self {
        self.m_projection = matrix;
        self
    }
    pub fn set_view_matrix(&mut self, matrix: Mat4) -> &mut Self {
        self.m_view.view_matrix = matrix;
        self
    }
    pub fn set_model_matrix(&mut self, matrix: Mat4) -> &mut Self {
        self.m_model = matrix;
        self
    }

    pub fn set_uniforms_tags(
        &mut self,
        proj_uniform: &str,
        view_uniform: &str,
        model: Option<&str>,
    ) -> &mut Self {
        self.uniforms_names = [
            CString::new(proj_uniform).unwrap(),
            CString::new(view_uniform).unwrap(),
            CString::new(model.unwrap_or("Model")).unwrap(),
        ];
        self
    }

    /// ### Development Requirement
    /// * implement auto-delete mechanism for
    pub fn add_program(&mut self, program_id: u32) -> &mut Self {
        let mut prog = ShaderUniforms::new(program_id);
        unsafe {
            gl::UseProgram(prog.program_id);
        }
        println!("{:?}", self.m_view.eye);
        prog.uniform_matrix4(&self.uniforms_names[0], 0, self.m_projection.as_ptr() as _);
        prog.uniform_matrix4(
            &self.uniforms_names[1],
            0,
            self.m_view.view_matrix.as_ptr() as _,
        );
        prog.uniform_matrix4(&self.uniforms_names[2], 0, self.m_model.as_ptr() as _);
        self.programs.push(prog);
        self
    }
    fn update_specific_uniforms(&mut self, uniforms: &mut ShaderUniforms) -> &mut Self {
        unsafe {
            gl::UseProgram(uniforms.program_id);
        }
        println!("{:?}", self.m_view.eye);
        uniforms.uniform_matrix4(&self.uniforms_names[0], 0, self.m_projection.as_ptr() as _);
        uniforms.uniform_matrix4(
            &self.uniforms_names[1],
            0,
            self.m_view.view_matrix.as_ptr() as _,
        );
        uniforms.uniform_matrix4(&self.uniforms_names[2], 0, self.m_model.as_ptr() as _);

        self
    }
    pub fn update_uniforms(&mut self, idx: Option<usize>) -> &mut Self {
        if let Some(valid_idx) = idx {
            let program = &mut self.programs[valid_idx];
            //Projection
            // println!("{:?}", self.uniforms_names);
            unsafe {
                gl::UseProgram(program.program_id);
            }
            println!("{:?}", self.m_view.eye);
            program.uniform_matrix4(&self.uniforms_names[0], 0, self.m_projection.as_ptr() as _);
            program.uniform_matrix4(
                &self.uniforms_names[1],
                0,
                self.m_view.view_matrix.as_ptr() as _,
            );
            program.uniform_matrix4(&self.uniforms_names[2], 0, self.m_model.as_ptr() as _);
        } else {
            for program in self.programs.iter_mut() {
                //Projection
                // println!("{:?}", self.uniforms_names);
                unsafe {
                    gl::UseProgram(program.program_id);
                }
                println!("{:?}", self.m_view.eye);
                program.uniform_matrix4(
                    &self.uniforms_names[0],
                    0,
                    self.m_projection.as_ptr() as _,
                );
                program.uniform_matrix4(
                    &self.uniforms_names[1],
                    0,
                    self.m_view.view_matrix.as_ptr() as _,
                );
                program.uniform_matrix4(&self.uniforms_names[2], 0, self.m_model.as_ptr() as _);
            }
        }
        self
    }

    pub fn update_camera(ev: &WindowEvent, camera_instance: &mut Camera) {
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
}

pub struct CameraProjection {
    projection_matrix: nalgebra_glm::Mat4,
}
impl Default for CameraProjection {
    fn default() -> Self {
        Self {
            projection_matrix: nalgebra_glm::Mat4::identity(),
        }
    }
}
impl CameraProjection {
    /// Creates a matrix for a right hand orthographic-view frustum with a depth range of -1 to 1
    ///
    /// # Parameters
    ///
    /// * `left` - Coordinate for left bound of matrix
    /// * `right` - Coordinate for right bound of matrix
    /// * `bottom` - Coordinate for bottom bound of matrix
    /// * `top` - Coordinate for top bound of matrix
    /// * `znear` - Distance from the viewer to the near clipping plane
    /// * `zfar` - Distance from the viewer to the far clipping plane
    ///
    pub fn normalized_ortho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            projection_matrix: nalgebra_glm::ortho(left, right, bottom, top, znear, zfar),
        }
    }

    /// Creates a matrix for a right hand perspective-view frustum with a depth range of -1 to 1
    ///
    /// # Parameters
    ///
    /// * `aspect` - Ratio of viewport width to height (width/height)
    /// * `fovy` - Field of view, in radians
    /// * `near` - Distance from the viewer to the near clipping plane
    /// * `far` - Distance from the viewer to the far clipping plane
    ///
    /// # Important note
    /// The `aspect` and `fovy` argument are interchanged compared to the original GLM API.
    pub fn perspective(aspect: f32, fovy: f32, near: f32, far: f32) -> Self {
        Self {
            projection_matrix: nalgebra_glm::perspective(aspect, fovy, near, far),
        }
    }
    pub fn default_ortho() -> Self {
        Self::normalized_ortho(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0)
    }
    pub fn default_perspective(aspect: f32, fov: f32) -> Self {
        Self::perspective(aspect, fov, 0.001, 100.0)
    }
}
#[derive(Clone, Copy)]
pub struct CameraView {
    view_matrix: nalgebra_glm::Mat4,
    eye: nalgebra_glm::Vec3,
    center: nalgebra_glm::Vec3,
    up: nalgebra_glm::Vec3,
}
impl Default for CameraView {
    fn default() -> Self {
        Self {
            view_matrix: nalgebra_glm::look_at(
                &nalgebra_glm::Vec3::new(1.0, 1.0, 1.0),
                &nalgebra_glm::Vec3::zeros(),
                &nalgebra_glm::Vec3::new(0.0, -0.001, 1.0),
            ),
            eye: nalgebra_glm::Vec3::new(1.0, 1.0, 1.0),
            center: nalgebra_glm::Vec3::zeros(),
            up: nalgebra_glm::Vec3::new(0.0, -0.001, 1.0),
        }
    }
}

impl CameraView {
    pub fn update_matrix(&mut self) -> &mut Self {
        self.view_matrix = nalgebra_glm::look_at(&self.eye, &self.center, &self.up);
        self
    }
    pub fn set_eye(&mut self, eye: &Vec3) -> &mut Self {
        self.eye = *eye;
        self
    }
    pub fn set_center(&mut self, center: &Vec3) -> &mut Self {
        self.center = *center;
        self
    }
    pub fn set_up(&mut self, up: &Vec3) -> &mut Self {
        self.up = *up;
        self
    }

    pub fn increase_altitude(&mut self, increment: f32) -> &mut Self {
        let rotation_axis = self.up.cross(&self.eye);
        let rotaiton_matrix = nalgebra_glm::rotation(increment, &rotation_axis);
        let mut tmp = Vec4::new(self.eye.x, self.eye.y, self.eye.z, 0.0);
        tmp = rotaiton_matrix * tmp;
        self.eye = Vec3::new(tmp.x, tmp.y, tmp.z);
        self
    }
    pub fn decrease_altitude(&mut self, decrement: f32) -> &mut Self {
        let rotation_axis = self.up.cross(&self.eye);
        let rotaiton_matrix = nalgebra_glm::rotation(-decrement, &rotation_axis);
        let mut tmp = Vec4::new(self.eye.x, self.eye.y, self.eye.z, 0.0);
        tmp = rotaiton_matrix * tmp;
        self.eye = Vec3::new(tmp.x, tmp.y, tmp.z);
        self
    }
    pub fn increase_azimuth(&mut self, increment: f32) -> &mut Self {
        let rotation_axis = self.up;
        let rotaiton_matrix = nalgebra_glm::rotation(increment, &rotation_axis);
        let mut tmp = Vec4::new(self.eye.x, self.eye.y, self.eye.z, 0.0);
        tmp = rotaiton_matrix * tmp;
        self.eye = Vec3::new(tmp.x, tmp.y, tmp.z);
        self
    }
    pub fn decrease_azimuth(&mut self, decrement: f32) -> &mut Self {
        let rotation_axis = self.up;
        let rotaiton_matrix = nalgebra_glm::rotation(-decrement, &rotation_axis);
        let mut tmp = Vec4::new(self.eye.x, self.eye.y, self.eye.z, 0.0);
        tmp = rotaiton_matrix * tmp;
        self.eye = Vec3::new(tmp.x, tmp.y, tmp.z);
        self
    }
}
