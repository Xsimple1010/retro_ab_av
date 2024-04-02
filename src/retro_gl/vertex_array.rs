use std::{mem::size_of, ptr::null};

use super::gl::gl::{self, types::*};

pub struct VertexArray {
    id: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Self { id }
    }

    pub fn set_attribute<V: Sized>(&self, atribute_pos: GLuint, components: GLint, offset: GLint) {
        unsafe {
            self.bind();
            gl::VertexAttribPointer(
                atribute_pos,
                components,
                gl::FLOAT,
                gl::FALSE,
                size_of::<V>().try_into().unwrap(),
                if offset == 0 {
                    null()
                } else {
                    offset as *const _
                },
            );
            gl::EnableVertexAttribArray(atribute_pos);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn un_bind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
