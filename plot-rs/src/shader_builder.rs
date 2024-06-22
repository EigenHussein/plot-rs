use std::{
    collections::HashMap,
    ffi::{c_void, CString},
};

pub struct ShaderUniforms {
    pub program_id: u32,
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
// pub struct GeometryShader;
#[derive(Default, Clone)]
pub enum ShaderAttributeType {
    #[default]
    Float,
    Vec2,
    Vec3,
    Vec4,
    Int,
    IVec2,
    IVec3,
    IVec4,
    UInt,
    UVec2,
    UVec3,
    UVec4,
    Mat2,
    Mat3,
    Mat4,
    Custom(&'static str),
}
impl ShaderAttributeType {
    pub fn to_glsl_type(&self) -> &'static str {
        match &self {
            ShaderAttributeType::Float => "float",
            ShaderAttributeType::Vec2 => "vec2",
            ShaderAttributeType::Vec3 => "vec3",
            ShaderAttributeType::Vec4 => "vec4",
            ShaderAttributeType::Int => "int",
            ShaderAttributeType::IVec2 => "ivec2",
            ShaderAttributeType::IVec3 => "ivec3",
            ShaderAttributeType::IVec4 => "ivec4",
            ShaderAttributeType::UInt => "uint",
            ShaderAttributeType::UVec2 => "uvec2",
            ShaderAttributeType::UVec3 => "uvec3",
            ShaderAttributeType::UVec4 => "uvec4",
            ShaderAttributeType::Mat2 => "mat2",
            ShaderAttributeType::Mat3 => "mat3",
            ShaderAttributeType::Mat4 => "mat4",
            ShaderAttributeType::Custom(t) => t,
        }
    }
}

#[derive(Clone)]
pub struct ShaderAttribute {
    pub prefix: Option<String>,
    pub type_: ShaderAttributeType,
    pub name: String,
}
impl ShaderAttribute {
    /// ### Description
    /// Generates the GLSL line corresponding to the attribute
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {};",
            self.prefix.clone().unwrap_or("".to_string()),
            self.type_.to_glsl_type(),
            self.name
        )
    }
    /// ### Description
    /// Generates a GLSL line for a vertex input attribute declaration.
    ///
    /// Example:
    /// ```
    /// use your_module_name::ShaderAttributeType;
    ///
    /// let vertex_input = new_vertex_input(0, "pos", ShaderAttributeType::Vec3);
    /// // Generates: "layout(location = 0) in vec3 pos;"
    /// ```
    pub fn new_input_attribute(
        input_attrib_index: Option<u32>,
        name: String,
        type_: ShaderAttributeType,
    ) -> Self {
        Self {
            prefix: Some(if let Some(id) = input_attrib_index {
                format!("layout (location = {} ) in", id.min(10))
            } else {
                "in".to_string()
            }),
            type_: type_,
            name: name.to_string(),
        }
    }
    /// ### Description
    /// Generates a GLSL line for a vertex output attribute declaration.
    ///
    /// Example:
    /// ```
    /// use your_module_name::ShaderAttributeType;
    ///
    /// // Output attribute with specified index and name
    /// let vertex_output1 = new_vertex_output(Some(0), "out_color".to_string(), ShaderAttributeType::Vec4);
    /// // Generates: "layout(location = 0) out vec4 out_color;"
    ///
    /// // Output attribute without location index (default behavior)
    /// let vertex_output2 = new_vertex_output(None, "out_position".to_string(), ShaderAttributeType::Vec3);
    /// // Generates: "out vec3 out_position;"
    /// ```
    pub fn new_output_attribute(
        output_attrib_index: Option<u32>,
        name: String,
        type_: ShaderAttributeType,
    ) -> Self {
        let prefix = if let Some(idx) = output_attrib_index {
            format!("layout (location = {} ) out", idx)
        } else {
            "out".to_string()
        };
        Self {
            prefix: Some(prefix),
            type_: type_,
            name: name.to_string(),
        }
    }
    pub fn generate_default_vertex_input_attributes() -> [ShaderAttribute; 4] {
        let attribs = [
            Self::new_input_attribute(Some(0), "position".to_string(), ShaderAttributeType::Vec3),
            Self::new_input_attribute(Some(1), "normal".to_string(), ShaderAttributeType::Vec3),
            Self::new_input_attribute(Some(2), "color".to_string(), ShaderAttributeType::Vec3),
            Self::new_input_attribute(Some(3), "txcoord".to_string(), ShaderAttributeType::Vec2),
        ];
        attribs
    }
    pub fn attrib_array_to_glsl(attribs: &[ShaderAttribute]) -> String {
        // let attribs =Self::generate_default_vertex_input_attributes();
        let attribs_strings: Vec<String> = attribs.iter().map(|a| a.to_string()).collect();
        attribs_strings.join("\n")
    }

    pub fn generate_uniform(
        output_attrib_index: Option<u32>,
        name: String,
        type_: ShaderAttributeType,
    ) -> Self {
        Self {
            prefix: Some(format!(
                "{} uniform",
                if let Some(id) = output_attrib_index {
                    format!("layout(location = {})", id)
                } else {
                    "".to_string()
                }
            )),
            type_: type_,
            name: name,
        }
    }

    pub fn generate_default_uniforms(model_matrix: bool) -> Vec<ShaderAttribute> {
        let mut uniforms: Vec<ShaderAttribute> = vec![
            Self::generate_uniform(None, "Proj".to_string(), ShaderAttributeType::Mat4),
            Self::generate_uniform(None, "View".to_string(), ShaderAttributeType::Mat4),
        ];

        if model_matrix == true {
            uniforms.push(Self::generate_uniform(
                None,
                "Model".to_string(),
                ShaderAttributeType::Mat4,
            ));
        }
        uniforms
        // let attribs_strings: Vec<String> = uniforms.iter().map(|a| a.to_string()).collect();
        // attribs_strings.join("\n")
    }
}

pub enum VertexTransformation {
    DirectMapping,
    ProjectionView,
    ProjectionViewModel,
    Custom(String),
}

impl VertexTransformation {
    pub fn to_string(&self) -> String {
        match self {
            VertexTransformation::ProjectionViewModel => {
                "gl_Position = Proj*View*Model*vec4(position.xyz,1);".to_string()
            }
            VertexTransformation::ProjectionView => {
                "gl_Position = Proj*View*vec4(position.xyz,1);".to_string()
            }
            VertexTransformation::DirectMapping => {
                "gl_Position = vec4(position.xyz,1);".to_string()
            }
            VertexTransformation::Custom(s) => s.clone(),
        }
    }
}

pub enum ColorAssignment {
    Solid(f32, f32, f32, f32),
    PerVertex,
    PerPosition,
    Texture,
    Custom(String),
}
impl ColorAssignment {
    pub fn to_string(&self) -> String {
        match self {
            ColorAssignment::Solid(r, g, b, a) => {
                format!("Color = vec4({}, {}, {}, {});", r, g, b, a)
            }
            ColorAssignment::PerPosition => "Color = vec4(0.5 + 0.5 * v_pos.xyz, 1);".to_string(),
            ColorAssignment::PerVertex => "Color = vec4(v_col.xyz, 1);".to_string(),
            ColorAssignment::Texture => "Color = vec4(texture(TX, txc).rgb, 1);".to_string(),
            ColorAssignment::Custom(s) => s.clone(),
        }
    }
}

pub struct ShaderBuilder {
    vertex_shader_input_attributes: Option<Vec<ShaderAttribute>>,
    //    vertex_fragment_passing;
    vertex_shader_uniforms: Vec<ShaderAttribute>,
    vertex_transformation: VertexTransformation,
    vertex_fragment_links: Vec<(ShaderAttribute, String)>,
    fragment_shader_uniforms: Vec<ShaderAttribute>,
    fragment_color_assignment: ColorAssignment,
}
impl ShaderBuilder {
    // pub fn new_builder() -> Self {}
    pub fn generate_vertex_shader(&self) -> String {
        let mut vertex_shader_source = r"
        #version 460 core
        // INPUT ATTRIBUTES 
        {INPUT_ATTRIBUTES}

        // OUTPUT ATTRIBUTES
        {OUTPUT_ATTRIBUTES}

        // UNIFORMS 
        {UNIFORMS}

        void main(){
            // VERTEX TRANSFORMATION 
            {VERTEX_TRANSFORMATION}

            // FRAGMENT PASSING 
            {FRAGMENT_PASSING}
        }
        "
        .to_string();

        // SET INPUT ATTRIBUTES
        vertex_shader_source = vertex_shader_source.replace(
            "{INPUT_ATTRIBUTES}",
            &ShaderAttribute::attrib_array_to_glsl(
                &self.vertex_shader_input_attributes.clone().unwrap_or(
                    ShaderAttribute::generate_default_vertex_input_attributes().to_vec(),
                ),
            ),
        );

        // SET UNIFORMS
        let uniforms: Vec<String> = self
            .vertex_shader_uniforms
            .iter()
            .map(|u| u.to_string())
            .collect();
        vertex_shader_source = vertex_shader_source.replace("{UNIFORMS}", &uniforms.join("\n"));
        vertex_shader_source = vertex_shader_source.replace(
            "{VERTEX_TRANSFORMATION}",
            &self.vertex_transformation.to_string(),
        );

        let output_attribs: Vec<String> = self
            .vertex_fragment_links
            .iter()
            .map(|sa| sa.0.to_string())
            .collect();

        let links: Vec<String> = self
            .vertex_fragment_links
            .iter()
            .map(|sa| format!("{} = {};", sa.0.name, sa.1))
            .collect();

        vertex_shader_source =
            vertex_shader_source.replace("{OUTPUT_ATTRIBUTES}", &output_attribs.join("\n"));
        vertex_shader_source =
            vertex_shader_source.replace("{FRAGMENT_PASSING}", &links.join("\n"));

        vertex_shader_source
    }

    pub fn generate_fragment_shader(&self) -> String {
        let mut fragment_shader_source = r"
        #version 460 core

        out vec4 Color; 

        // INPUT ATTRIBUTES
        {INPUT_ATTRIBUTES}

        // UNIFORMS 
        {UNIFORMS}

        void main(){
            // COLOR ASSIGNMENT 
            {COLOR_ASSIGNMENT}
        }
        "
        .to_string();

        // FRAGMENT SHADER INPUT ATTRIBUTES
        let frag_inp_attribs: Vec<String> = self
            .vertex_fragment_links
            .iter()
            .map(|va| {
                ShaderAttribute::new_input_attribute(None, va.0.name.clone(), va.0.type_.clone())
                    .to_string()
            })
            .collect();
        fragment_shader_source =
            fragment_shader_source.replace("{INPUT_ATTRIBUTES}", &frag_inp_attribs.join("\n"));

        // SET UNIFORMS
        let uniforms: Vec<String> = self
            .fragment_shader_uniforms
            .iter()
            .map(|u| u.to_string())
            .collect();

        fragment_shader_source = fragment_shader_source.replace("{UNIFORMS}", &uniforms.join("\n"));
        fragment_shader_source = fragment_shader_source.replace(
            "{COLOR_ASSIGNMENT}",
            &self.fragment_color_assignment.to_string(),
        );
        fragment_shader_source
    }

    pub fn compile_shader(&self) -> RawShader {
        RawShader(shader::CompileProgramFromString(
            self.generate_vertex_shader(),
            self.generate_fragment_shader(),
            None,
        ))
    }

    pub fn set_vertex_shader_input_attributes(
        &mut self,
        vertex_shader_input_attribs: Option<Vec<ShaderAttribute>>,
    ) -> &mut Self {
        self.vertex_shader_input_attributes = vertex_shader_input_attribs;
        self
    }

    pub fn add_vertex_uniform(&mut self, uniform: ShaderAttribute) -> &mut Self {
        self.vertex_shader_uniforms.push(uniform);
        self
    }
    pub fn set_vertex_transformation(
        &mut self,
        vertex_transformation: VertexTransformation,
    ) -> &mut Self {
        self.vertex_transformation = vertex_transformation;
        self
    }

    pub fn add_vertex2fragment_link(
        &mut self,
        inout_attrib: ShaderAttribute,
        vertex_variable: String,
    ) -> &mut Self {
        self.vertex_fragment_links
            .push((inout_attrib, vertex_variable));
        self
    }

    pub fn add_fragment_uniform(&mut self, uniform: ShaderAttribute) -> &mut Self {
        self.fragment_shader_uniforms.push(uniform);
        self
    }
    pub fn set_color_assignment(&mut self, new_color_assignment: ColorAssignment) -> &mut Self {
        self.fragment_color_assignment = new_color_assignment;
        self
    }
}

impl Default for ShaderBuilder {
    fn default() -> Self {
        Self {
            vertex_shader_input_attributes: None,
            vertex_shader_uniforms: ShaderAttribute::generate_default_uniforms(false),
            vertex_transformation: VertexTransformation::ProjectionView,
            vertex_fragment_links: [(
                ShaderAttribute::new_output_attribute(
                    None,
                    "v_pos".to_string(),
                    ShaderAttributeType::Vec3,
                ),
                "position".to_string(),
            )]
            .to_vec(),
            fragment_shader_uniforms: Vec::new(),
            fragment_color_assignment: ColorAssignment::Solid(1.0, 0.5, 0.25, 1.0),
        }
    }
}

pub struct RawShader(u32);
impl RawShader {
    pub fn get_id(&self) -> u32 {
        self.0
    }
}
impl Drop for RawShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }
}

pub mod shader {
    use crate::core::read_both;

    // use crate::GetCompiledShader;
    #[allow(non_snake_case)]
    // pub fn GetCompiledShader(shader_type: GLenum, shader_source: &String) -> GLuint {
    pub fn GetCompiledShader(
        shader_type: gl::types::GLenum,
        shader_source: &String,
    ) -> gl::types::GLuint {
        // let source_ptr = shader_source as *const GLchar;
        let sources = vec![shader_source.as_ptr() as *const i8];
        let sources_len = vec![shader_source.len() as i32];
        // let source_len = vec![]
        let mut shader: u32 = 0;
        unsafe {
            shader = gl::CreateShader(shader_type);
            // let source_ptr2 = &source_ptr;
            gl::ShaderSource(shader, 1, sources.as_ptr(), sources_len.as_ptr());
            gl::CompileShader(shader);

            //error checking
            let mut err_msg_len: i32 = 0;
            let mut err_msg: Vec<u8> = Vec::new();
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut err_msg_len);
            if err_msg_len == 0 {
                println!("Error Compiling shader :{:x}", shader_type);
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut err_msg_len);
                err_msg.resize(err_msg_len as usize, 0);
                gl::GetShaderInfoLog(
                    shader,
                    err_msg_len,
                    &mut err_msg_len,
                    (&mut err_msg[0]) as *mut u8 as _,
                );
                let k = String::from_utf8(err_msg).unwrap();
                println!("Error:{}", k);
                return 0;
            }
        }
        println!("Done Compiling...");
        return shader;
    }
    #[allow(non_snake_case)]
    pub fn CompileProgram(
        vertex_filename: &str,
        fragment_filename: &str,
        geometry_filename: Option<&str>,
    ) -> gl::types::GLuint {
        let (v_source, f_source) = read_both(vertex_filename, fragment_filename);
        unsafe {
            let vertex = GetCompiledShader(gl::VERTEX_SHADER, &v_source);
            let fragment = GetCompiledShader(gl::FRAGMENT_SHADER, &f_source);

            let Prog = gl::CreateProgram();
            gl::AttachShader(Prog, vertex);
            gl::AttachShader(Prog, fragment);
            if let Some(geometry_shader_filename) = geometry_filename {
                let g_shader = std::fs::read_to_string(geometry_shader_filename)
                    .expect("Couldn't read geometry shader!");
                let geometry = GetCompiledShader(gl::GEOMETRY_SHADER, &g_shader);
                gl::AttachShader(Prog, geometry);
                gl::LinkProgram(Prog);
                gl::DeleteShader(geometry);
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            } else {
                gl::LinkProgram(Prog);
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            }
            return Prog;
        }
    }

    #[allow(non_snake_case)]
    pub fn CompileProgramFromString(
        vertex_source: String,
        fragment_source: String,
        geometry_source: Option<String>,
    ) -> gl::types::GLuint {
        let (v_source, f_source) = (vertex_source, fragment_source); //super::read_both(vertex_filename, fragment_filename);
        unsafe {
            let vertex = GetCompiledShader(gl::VERTEX_SHADER, &v_source);
            let fragment = GetCompiledShader(gl::FRAGMENT_SHADER, &f_source);

            let Prog = gl::CreateProgram();
            gl::AttachShader(Prog, vertex);
            gl::AttachShader(Prog, fragment);
            if let Some(g_shader) = geometry_source {
                // let g_shader = std::fs::read_to_string(geometry_shader_filename)
                // .expect("Couldn't read geometry shader!");
                let geometry = GetCompiledShader(gl::GEOMETRY_SHADER, &g_shader);
                gl::AttachShader(Prog, geometry);
                gl::LinkProgram(Prog);
                gl::DeleteShader(geometry);
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            } else {
                gl::LinkProgram(Prog);
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            }
            return Prog;
        }
    }

    pub fn compute_shader(cshader_source: &String) -> u32 {
        unsafe {
            let COMP = gl::CreateProgram();
            let C = GetCompiledShader(gl::COMPUTE_SHADER, cshader_source); //CompileShaderSource(gl::COMPUTE_SHADER, &cshader);
            gl::AttachShader(COMP, C);
            gl::LinkProgram(COMP);
            gl::DeleteShader(C);
            return COMP;
        }
    }
}
