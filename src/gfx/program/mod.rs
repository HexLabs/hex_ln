use crate::gfx::{gl::*, shader::Shader, Resource};

pub struct Program(GLuint);

impl Program {
    pub fn new(vert_src: &str, frag_src: &str) -> Program {
        let prog = unsafe { glCreateProgram() };

        let vert = Shader::new(vert_src, GL_VERTEX_SHADER);
        let frag = Shader::new(frag_src, GL_FRAGMENT_SHADER);
        vert.attach(prog);
        frag.attach(prog);

        unsafe {
            glLinkProgram(prog);
        }

        Program(prog)
    }
}

impl Resource for Program {
    fn bind(&self) {
        unsafe {
            glUseProgram(self.0);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            glDeleteProgram(self.0);
        }
    }
}
