mod defaults;

use crate::gfx::*;
pub use defaults::*;

pub struct Shader(GLuint);

impl Shader {
    pub fn new(source: &str, stage: GLenum) -> Shader {
        unsafe {
            let shader = glCreateShader(stage);
            glShaderSource(shader, 1, &(source.as_ptr() as _), core::ptr::null());
            glCompileShader(shader);

            Shader(shader)
        }
    }

    pub fn attach(&self, prog: GLuint) {
        unsafe {
            glAttachShader(prog, self.0);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            glDeleteShader(self.0);
        }
    }
}
