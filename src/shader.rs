use cgmath::{ Vector1, Vector2, Vector3, Matrix4, prelude::* };
use gl::types::{ GLuint, GLchar, GLint, GLboolean };
use std::ffi::CString;

/// Different type of shader
/// that we can compile.
pub enum ShaderType {
    Vertex,
    Fragment,
}

/// A shader error.
pub struct ShaderError {
    /// Kind of error.
    pub kind: String,

    /// The error description.
    pub message: String,
}

impl ShaderError {
    /// Create new shader error.
    /// 
    /// # Arguments
    /// * `kind` - The kind of error.
    /// * `message` - The error description.
    pub fn new(kind: &str, message: &str) -> ShaderError {
        ShaderError {
            kind: String::from(kind),
            message: String::from(message),
        }
    }
}

impl From<std::io::Error> for ShaderError {
    /// Convert an `std::io::Error` to a `ShaderError`
    /// 
    /// # Arguments
    /// * `error` - The error to convert.
    fn from(error: std::io::Error) -> Self { 
        ShaderError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}


pub struct Shader {
    /// The shader program id.
    program: u32,
}

impl Shader {
    /// Returns a shader.
    /// 
    /// # Arguments
    /// * `vertex_shader_path` - The path of the vertex shader.
    /// * `fragment_shader_path` - The path of the fragment shader.
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Result<Shader, ShaderError> {
        unsafe {
            let vs_src = std::fs::read_to_string(vertex_shader_path)?;
            let fs_src = std::fs::read_to_string(fragment_shader_path)?;

            let vs = Shader::compile_shader(&vs_src, &ShaderType::Vertex)?;
            let fs = Shader::compile_shader(&fs_src, &ShaderType::Fragment)?;

            let program = Shader::create_and_link_program(vs, fs)?;

            Ok(Shader { program: program })
        }
    }

    /// Just return the shader program id.
    pub fn get_program(&self) -> GLuint {
        self.program
    }

    /// Compile a shader.
    /// 
    /// # Arguments
    /// * `src` - The shader source that we want to compile.
    /// * `shader_type` - The type of shader.
    pub unsafe fn compile_shader(src: &str, shader_type: &ShaderType) -> Result<u32, ShaderError> {
        
        let shader = gl::CreateShader(match shader_type { 
            ShaderType::Fragment => gl::FRAGMENT_SHADER, 
            ShaderType::Vertex   => gl::VERTEX_SHADER,
        });

        let c_str_shader = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str_shader.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut infos = Vec::with_capacity(len as usize);
            infos.set_len((len as usize) - 1);

            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), infos.as_mut_ptr() as *mut GLchar);
            
            let shader_type_name = match shader_type { 
                ShaderType::Vertex   => "Vertex", 
                ShaderType::Fragment => "Fragment",
            };

            let shader_result_str = std::str::from_utf8(infos.as_slice()).unwrap();
            let msg = format!("{} shader compilation error:\n{}", shader_type_name, shader_result_str);

            Err(ShaderError::new("Shader", &msg))
        } else {
            Ok(shader)
        }
    }

    /// Create program id and link vertex & fragment shader to it.
    /// 
    /// # Arguments
    /// * `vertex_shader` - The vertex shader ID.
    /// * `fragment_shader` - The fragment shader ID.
    pub unsafe fn create_and_link_program(vertex_shader: u32, fragment_shader: u32) -> Result<u32, ShaderError> {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader as GLuint);
        gl::AttachShader(program, fragment_shader as GLuint);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut infos = Vec::with_capacity(len as usize);
            infos.set_len((len as usize) - 1);

            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), infos.as_mut_ptr() as *mut GLchar);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let shader_result_str = std::str::from_utf8(infos.as_slice()).unwrap();
            let msg = format!("Program shader compilation error:\n{}", shader_result_str);

            Err(ShaderError::new("Program", &msg))
        } else {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(program)
        }
    }

    /// Set an uniform (`mat4`) value to shaders.
    /// 
    /// # Arguments
    /// * `name` - The name of the uniform variable (in the shaders) to set.
    /// * `value` - The new value of the uniform variable.
    /// * `transpose` - `true` if the matrix must be transposed otherwise `false`.
    pub fn set_matrix4(&self, name: &str, value: &Matrix4<f32>, transpose: bool) {
        unsafe {
            let name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::UniformMatrix4fv(location, 1, transpose as GLboolean, value.as_ptr());
        }
    }

    /// Set an uniform (`float`) value to shaders.
    /// 
    /// # Arguments
    /// * `name` - The name of the uniform variable (in the shader) to set.
    /// * `value` - The new value of the uniform variable.
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    /// Set an uniform (`vector1`) values to shaders.
    /// 
    /// # Arguments
    /// * `name` - The name of the uniform variable (in the shader) to set.
    /// * `values` - The new value of the uniform variable.
    pub fn set_vec1(&self, name: &str, values: &[Vector1<f32>]) {
        unsafe {
            let name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform1fv(location, values.len() as i32, &values[0] as *const Vector1<f32> as *const f32);
        }
    }

    /// Set an uniform (`vector2`) values to shaders.
    /// 
    /// # Arguments
    /// * `name` - The name of the uniform variable (in the shader) to set.
    /// * `values` - The new value of the uniform variable.
    pub fn set_vec2(&self, name: &str, values: &[Vector2<f32>]) {
        unsafe {
            let name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform2fv(location, values.len() as i32, &values[0] as *const Vector2<f32> as *const f32);
        }
    }

    /// Set an uniform (`vector3`) values to shaders.
    /// 
    /// # Arguments
    /// * `name` - The name of the uniform variable (in the shader) to set.
    /// * `values` - The new value of the uniform variable.
    pub fn set_vec3(&self, name: &str, values: &[Vector3<f32>]) {
        unsafe {
            let name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::Uniform3fv(location, values.len() as i32, &values[0] as *const Vector3<f32> as *const f32);
        }
    }
}

