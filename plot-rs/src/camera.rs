use std::{collections::HashMap, ffi::{c_void, CString}};

use nalgebra_glm::{Mat4, Vec3, Vec4};


pub struct ShaderUniforms {
    program_id: u32,
    lookup: HashMap<CString, i32>,
}

impl ShaderUniforms {
    pub fn new(program_id: u32) -> Self {
        Self {
            program_id: program_id,
            lookup: HashMap::new(),
        }
    }
    pub fn uniform_1f(&mut self, name: &CString, value: f32) -> &mut Self {
        let mut uniform_id = 0i32;
        if self.lookup.contains_key(name) {
            uniform_id = self.lookup[name];
        } else {
            uniform_id = unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as _) };
            self.lookup.insert(name.clone(), uniform_id);
        }
        unsafe {
            gl::UseProgram(self.program_id);
            gl::Uniform1f(uniform_id, value);
        }
        self
    }
    pub fn uniform_2f(&mut self, name: &CString, value: (f32, f32)) -> &mut Self {
        let mut uniform_id = 0i32;
        if self.lookup.contains_key(name) {
            uniform_id = self.lookup[name];
        } else {
            uniform_id = unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as _) };
            self.lookup.insert(name.clone(), uniform_id);
        }
        unsafe {
            gl::UseProgram(self.program_id);
            gl::Uniform2f(uniform_id, value.0, value.1);
        }
        self
    }
    pub fn uniform_3f(&mut self, name: &CString, value: (f32, f32, f32)) -> &mut Self {
        let mut uniform_id = 0i32;
        if self.lookup.contains_key(name) {
            uniform_id = self.lookup[name];
        } else {
            uniform_id = unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as _) };
            self.lookup.insert(name.clone(), uniform_id);
        }
        unsafe {
            gl::UseProgram(self.program_id);
            gl::Uniform3f(uniform_id, value.0, value.1, value.2);
        }
        self
    }

    pub fn uniform_1fv(&mut self, name: &CString, value: *const c_void) -> &mut Self {
        let mut uniform_id = 0i32;
        if self.lookup.contains_key(name) {
            uniform_id = self.lookup[name];
        } else {
            uniform_id = unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as _) };
            self.lookup.insert(name.clone(), uniform_id);
        }
        unsafe {
            gl::UseProgram(self.program_id);
            gl::Uniform1fv(uniform_id, 1, value as _);
        }
        self
    }
    pub fn uniform_2fv(&mut self, name: &CString, value: *const c_void) -> &mut Self {
        let mut uniform_id = 0i32;
        if self.lookup.contains_key(name) {
            uniform_id = self.lookup[name];
        } else {
            uniform_id = unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as _) };
            self.lookup.insert(name.clone(), uniform_id);
        }
        unsafe {
            gl::UseProgram(self.program_id);
            gl::Uniform2fv(uniform_id, 1, value as _);
        }
        self
    }
    pub fn uniform_3fv(&mut self, name: &CString, value: *const c_void) -> &mut Self {
        let mut uniform_id = 0i32;
        if self.lookup.contains_key(name) {
            uniform_id = self.lookup[name];
        } else {
            uniform_id = unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as _) };
            self.lookup.insert(name.clone(), uniform_id);
        }
        unsafe {
            gl::UseProgram(self.program_id);
            gl::Uniform3fv(uniform_id, 1, value as _);
        }
        self
    }
    pub fn uniform_4fv(&mut self, name: &CString, value: *const c_void) -> &mut Self {
        let mut uniform_id = 0i32;
        if self.lookup.contains_key(name) {
            uniform_id = self.lookup[name];
        } else {
            uniform_id = unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as _) };
            self.lookup.insert(name.clone(), uniform_id);
        }
        if uniform_id < 0 {
            return self;
        }
        unsafe {
            gl::UseProgram(self.program_id);
            println!("{:?}", (uniform_id, name));
            gl::Uniform4fv(uniform_id, 1, value as _);
        }
        self
    }

    pub fn uniform_matrix4(
        &mut self,
        name: &CString,
        tanspose: u8,
        value: *const c_void,
    ) -> &mut Self {
        let mut uniform_id = 0i32;
        if self.lookup.contains_key(name) {
            uniform_id = self.lookup[name];
        } else {
            uniform_id = unsafe { gl::GetUniformLocation(self.program_id, name.as_ptr() as _) };
            self.lookup.insert(name.clone(), uniform_id);
        }
        unsafe {
            gl::UseProgram(self.program_id);
            gl::UniformMatrix4fv(uniform_id, 1, tanspose, value as _);
        }
        self
    }
}

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
            m_view: CameraView {
                view_matrix: nalgebra_glm::Mat4::identity(),
                ..Default::default()
            },
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
        self.programs.push(ShaderUniforms::new(program_id));
        self
    }

    pub fn update_uniforms(&mut self) -> &mut Self {
        for program in self.programs.iter_mut() {
            //Projection
            // println!("{:?}", self.uniforms_names);
            unsafe {
                gl::UseProgram(program.program_id);
            }
            program.uniform_matrix4(&self.uniforms_names[0], 0, self.m_projection.as_ptr() as _);
            program.uniform_matrix4(
                &self.uniforms_names[1],
                0,
                self.m_view.view_matrix.as_ptr() as _,
            );
            program.uniform_matrix4(&self.uniforms_names[2], 0, self.m_model.as_ptr() as _);
        }
        self
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
        println!("{:?}", self.eye);
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
