use super::*;
pub struct Vao {
    pub gl_handle: types::GLuint,
    /// Store a Rc to the Gl instance to ensure that we can destroy this resource when dropped
    gl: std::rc::Rc<Gl>,
}

impl Vao {
    pub fn new(
        pointer_definitions: &Vec<VertexAttribPointerDefinition>,
        gl: std::rc::Rc<Gl>,
    ) -> Vao {
        let mut gl_handle: types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut gl_handle as *mut _);
        }
        assert!(gl_handle > 0);
        unsafe {
            gl.BindVertexArray(gl_handle);
        }

        for def in pointer_definitions {
            let attr_def = &def.attribute_definition;

            def.vbo.bind(&gl);

            unsafe {
                gl.EnableVertexAttribArray(def.attribute_index);

                match (
                    def.attribute_definition.data_type,
                    def.attribute_definition.normalized,
                ) {
                    (FLOAT, _) | (_, true) => {
                        gl.VertexAttribPointer(
                            def.attribute_index,
                            attr_def.dimension_count as i32,
                            attr_def.data_type,
                            attr_def.normalized as u8,
                            attr_def.stride as i32,
                            attr_def.offset as *const types::GLvoid,
                        );
                    }
                    (UNSIGNED_SHORT, _) => {
                        gl.VertexAttribIPointer(
                            def.attribute_index,
                            attr_def.dimension_count as i32,
                            attr_def.data_type,
                            attr_def.stride as i32,
                            attr_def.offset as *const types::GLvoid,
                        );
                    }
                    _ => unimplemented!("unsupported vertex attribute type"),
                };
            }
        }
        unsafe {
            gl.BindVertexArray(0);
        }

        ArrayVbo::unbind(&gl);

        Vao { gl_handle, gl }
    }
    pub fn bind(&self, gl: &super::Gl) {
        unsafe {
            gl.BindVertexArray(self.gl_handle);
        }
    }
    pub fn unbind(gl: &super::Gl) {
        unsafe {
            gl.BindVertexArray(0);
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &self.gl_handle as *const _);
        }
    }
}
pub struct VertexAttributeDefinition {
    pub dimension_count: u8,
    pub data_type: u32,
    pub normalized: bool,
    pub stride: u32,
    pub offset: usize,
}
pub struct VertexAttribPointerDefinition<'a> {
    vbo: &'a ArrayVbo,
    attribute_index: u32,
    attribute_definition: VertexAttributeDefinition,
}

impl<'a> VertexAttribPointerDefinition<'a> {
    pub fn new(
        vbo: &'a ArrayVbo,
        attribute_index: u32,
        attribute_definition: VertexAttributeDefinition,
    ) -> Self {
        Self {
            vbo,
            attribute_index,
            attribute_definition,
        }
    }
}
