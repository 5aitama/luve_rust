use cgmath::{ Matrix4, prelude::* };
use gl::types::{ GLuint, GLchar, GLint, GLboolean };
use std::ffi::CString;

#[repr(u32)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct ShaderError {
    pub kind: String,
    pub message: String,
}

impl ShaderError {
    pub fn new(kind: &str, message: &str) -> ShaderError {
        ShaderError {
            kind: String::from(kind),
            message: String::from(message),
        }
    }
}

impl From<std::io::Error> for ShaderError {
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
        let mut infos = Vec::with_capacity(512);
        infos.set_len(512 - 1);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), infos.as_mut_ptr() as *mut GLchar);
            
            let shader_type_name = match shader_type { 
                ShaderType::Vertex   => "Vertex", 
                ShaderType::Fragment => "Fragment",
            };

            let msg = format!("{} shader compilation error:\n{}", shader_type_name, std::str::from_utf8(&infos).unwrap());
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

    pub fn set_matrix4(&self, name: &str, value: &Matrix4<f32>, transpose: bool) {
        unsafe {
            let name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.program, name.as_ptr());
            gl::UniformMatrix4fv(location, 1, transpose as GLboolean, value.as_ptr());
        }
    }
}