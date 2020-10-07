#![allow(clippy::all)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;

pub use self::camera::*;
pub use self::color::*;
pub use self::shader::*;
pub use self::vao::*;
pub use self::vbo::*;

mod camera;
mod color;
mod shader;
mod vao;
mod vbo;

pub fn clear(c: &color::Color, gl: &Gl) {
    unsafe {
        gl.ClearColor(c.r(), c.g(), c.b(), c.a());
        gl.Clear(DEPTH_BUFFER_BIT | crate::gl::COLOR_BUFFER_BIT);
    }
}

pub fn draw_arrays(vertex_count: i32, gl: &Gl) {
    unsafe {
        gl.DrawArrays(TRIANGLES, 0, vertex_count);
    }
}

pub fn get_uniform_location(shader: &Shader, name: &str, gl: &Gl) -> Result<i32, anyhow::Error> {
    let c_uniform_name = CString::new(name)?;
    let location =
        unsafe { gl.GetUniformLocation(shader.program_gl_handle, c_uniform_name.as_ptr()) };
    Ok(location)
}