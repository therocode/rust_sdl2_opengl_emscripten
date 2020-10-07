use super::*;

pub struct ArrayVbo {
    gl_handle: u32,
    gl: std::rc::Rc<Gl>,
}

impl ArrayVbo {
    pub fn new(gl: std::rc::Rc<Gl>) -> Self {
        let mut gl_handle: types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut gl_handle as *mut _);
        }
        assert!(gl_handle > 0);
        Self { gl_handle, gl }
    }

    pub fn bind(&self, gl: &super::Gl) {
        unsafe {
            gl.BindBuffer(super::ARRAY_BUFFER, self.gl_handle);
        }
    }
    pub fn unbind(gl: &super::Gl) {
        unsafe {
            gl.BindBuffer(super::ARRAY_BUFFER, 0);
        }
    }
    pub fn upload_array_vbo_vec<T>(
        &self,
        usage: crate::gl::types::GLenum,
        data: &Vec<T>,
        gl: &crate::gl::Gl,
    ) {
        let s = data.as_slice();
        unsafe {
            let bytes = std::slice::from_raw_parts(
                s.as_ptr() as *const u8,
                s.len() * std::mem::size_of::<T>(),
            );

            self.upload_array_vbo_raw(usage, bytes, gl)
        }
    }
    pub fn upload_array_vbo_raw(
        &self,
        usage: crate::gl::types::GLenum,
        bytes: &[u8],
        gl: &crate::gl::Gl,
    ) {
        self.bind(gl);
        unsafe {
            gl.BufferData(
                crate::gl::ARRAY_BUFFER,
                bytes.len() as isize,
                bytes.as_ptr() as *const std::ffi::c_void,
                usage,
            );
        }
    }
}

impl Drop for ArrayVbo {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.gl_handle as *const _);
        }
    }
}
