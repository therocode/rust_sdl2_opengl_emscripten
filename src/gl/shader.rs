pub use super::*;

// shader
pub struct Shader {
    pub program_gl_handle: types::GLuint,
    pub vertex_gl_handle: types::GLuint,
    pub fragment_gl_handle: types::GLuint,
    pub vertex_source: String,
    pub fragment_source: String,
    /// Store a Rc to the Gl instance to ensure that we can destroy this resource when dropped
    gl: std::rc::Rc<Gl>,
}

pub struct ShaderAttributeBinding {
    pub name: String,
    pub index: u32,
}

#[derive(Debug)]
pub struct ShaderCompilationError {
    pub log: String,
}

#[derive(Debug)]
pub enum ShaderError {
    ///null terminator in string for example
    VertexSourceMalformed,
    FragmentSourceMalformed,
    AttributeStringMalformed,
    VertexShaderCompilationFailed(ShaderCompilationError),
    FragmentShaderCompilationFailed(ShaderCompilationError),
}

impl std::error::Error for ShaderError {}
impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            ShaderError::VertexSourceMalformed => {
                String::from("Vertex input string is malformed. Contains zero bytes maybe")
            }
            ShaderError::FragmentSourceMalformed => {
                String::from("Fragment input string is malformed. Contains zero bytes maybe")
            }
            ShaderError::AttributeStringMalformed => {
                String::from("Attribute name input string is malformed. Contains zero bytes maybe")
            }
            ShaderError::VertexShaderCompilationFailed(l) => {
                std::format!("Vertex shader failed to compile: {}", l.log)
            }
            ShaderError::FragmentShaderCompilationFailed(l) => {
                std::format!("Fragment shader failed to compile: {}", l.log)
            }
        };
        write!(f, "{}", to_write)
    }
}

impl Shader {
    pub fn new(
        vertex_source: &str,
        fragment_source: &str,
        attribute_bindings: &[ShaderAttributeBinding],
        gl: std::rc::Rc<Gl>,
    ) -> Result<Self, ShaderError> {
        //compile vertex shader
        let vertex_gl_handle = unsafe { gl.CreateShader(VERTEX_SHADER) };
        let source = std::ffi::CString::new(vertex_source)
            .map_err(|_| ShaderError::VertexSourceMalformed)?;
        unsafe { gl.ShaderSource(vertex_gl_handle, 1, &source.as_ptr(), std::ptr::null()) };
        unsafe { gl.CompileShader(vertex_gl_handle) };

        let mut success: types::GLint = 0;
        unsafe { gl.GetShaderiv(vertex_gl_handle, COMPILE_STATUS, &mut success as *mut i32) };

        if success as u8 == FALSE {
            let mut log_size: types::GLint = 0;
            unsafe { gl.GetShaderiv(vertex_gl_handle, INFO_LOG_LENGTH, &mut log_size as *mut i32) };

            let mut error_log: Vec<types::GLchar> = Vec::with_capacity(log_size as usize);
            error_log.resize_with(log_size as usize, Default::default);

            unsafe {
                gl.GetShaderInfoLog(
                    vertex_gl_handle,
                    log_size,
                    &mut log_size as *mut i32,
                    error_log.as_ptr() as *mut _,
                )
            };

            let error_log = error_log
                .into_iter()
                .map(|c| c as u8 as char)
                .collect::<String>();

            unsafe {
                gl.DeleteShader(vertex_gl_handle);
            }

            return Err(ShaderError::VertexShaderCompilationFailed(
                ShaderCompilationError { log: error_log },
            ));
        }

        //compile fragment shader
        let fragment_gl_handle = unsafe { gl.CreateShader(FRAGMENT_SHADER) };
        let source = std::ffi::CString::new(fragment_source)
            .map_err(|_| ShaderError::FragmentSourceMalformed)?;
        unsafe { gl.ShaderSource(fragment_gl_handle, 1, &source.as_ptr(), std::ptr::null()) };
        unsafe { gl.CompileShader(fragment_gl_handle) };

        let mut success: types::GLint = 0;
        unsafe { gl.GetShaderiv(fragment_gl_handle, COMPILE_STATUS, &mut success as *mut i32) };

        if success as u8 == FALSE {
            let mut log_size: types::GLint = 0;
            unsafe {
                gl.GetShaderiv(
                    fragment_gl_handle,
                    INFO_LOG_LENGTH,
                    &mut log_size as *mut i32,
                )
            };

            let mut error_log: Vec<types::GLchar> = Vec::with_capacity(log_size as usize);
            error_log.resize_with(log_size as usize, Default::default);

            unsafe {
                gl.GetShaderInfoLog(
                    fragment_gl_handle,
                    log_size,
                    &mut log_size as *mut i32,
                    error_log.as_ptr() as *mut _,
                )
            };

            let error_log = error_log
                .into_iter()
                .map(|c| c as u8 as char)
                .collect::<String>();

            unsafe {
                gl.DeleteShader(vertex_gl_handle);
                gl.DeleteShader(fragment_gl_handle);
            }

            return Err(ShaderError::FragmentShaderCompilationFailed(
                ShaderCompilationError { log: error_log },
            ));
        }

        //assemble shader program
        let program_gl_handle = unsafe { gl.CreateProgram() };
        unsafe {
            gl.AttachShader(program_gl_handle, vertex_gl_handle);
            gl.AttachShader(program_gl_handle, fragment_gl_handle);
            gl.LinkProgram(program_gl_handle);
        }

        for binding in attribute_bindings {
            let name = std::ffi::CString::new(&binding.name[..])
                .map_err(|_| ShaderError::AttributeStringMalformed)?;
            unsafe { gl.BindAttribLocation(program_gl_handle, binding.index, name.as_ptr()) }
        }

        Ok(Self {
            program_gl_handle,
            vertex_gl_handle,
            fragment_gl_handle,
            vertex_source: String::from(vertex_source),
            fragment_source: String::from(fragment_source),
            gl,
        })
    }
    pub fn bind(&self, gl: &Gl) {
        unsafe {
            gl.UseProgram(self.program_gl_handle);
        }
    }
    pub fn unbind(gl: &Gl) {
        unsafe {
            gl.UseProgram(0);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program_gl_handle);
            self.gl.DeleteShader(self.vertex_gl_handle);
            self.gl.DeleteShader(self.fragment_gl_handle);
        }
    }
}

pub fn new_default_shader(gl: std::rc::Rc<crate::gl::Gl>) -> crate::gl::Shader {
    crate::gl::Shader::new(
        VERTEX_SOURCE,
        FRAGMENT_SOURCE,
        &[
            ShaderAttributeBinding {
                name: String::from("position"),
                index: 0,
            },
            ShaderAttributeBinding {
                name: String::from("color"),
                index: 1,
            },
        ],
        gl,
    )
    .unwrap() //unwrap since the inputs are hard coded
}

const VERTEX_SOURCE: &str = "#version 300 es

layout(location=0) in vec4 position; //will be compatible with vec2 and vec3 attributes. extended to vec4: (0, 0, 0, 1) for missing components
layout(location=1) in vec4 color;

uniform mat4 view_projection;

out vec4 v_color;

void main()
{
    gl_Position = view_projection * position;
    v_color = color;
}
";

const FRAGMENT_SOURCE: &str = "#version 300 es

precision mediump float;

in vec4 v_color;

out vec4 out_color;

void main()
{
    out_color = v_color;
}
";

pub const DEFAULT_PROJECTION_UNIFORM: &str = "view_projection";
