use gl;
use gl::types::*;

#[derive(Debug, Default)]
pub struct Vao {
    id: GLuint,
}

impl Drop for Vao {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Vao {
    pub fn gen(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.id);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn un_bind(&self) {
        todo!()
    }

    pub fn destroy(&self) {
        todo!()
    }
}
