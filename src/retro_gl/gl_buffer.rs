use gl::types::GLuint;
use std::mem::size_of_val;

pub struct GlBuffer {
    id: GLuint,
    target: GLuint,
}

impl Drop for GlBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl GlBuffer {
    pub fn new(target: GLuint) -> GlBuffer {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Self { id, target }
    }

    pub fn set_data<T>(&self, data: [T; 4]) {
        unsafe {
            self.bind();
            gl::BufferData(
                self.target,
                size_of_val(&data) as isize,
                data.as_ptr().cast(),
                gl::STREAM_DRAW,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    pub fn un_bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }
    }
}
