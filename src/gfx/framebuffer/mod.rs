use crate::gfx::{
    texture::{Format, Texture},
    *,
};

pub const SWAP_CHAIN: Framebuffer = Framebuffer(0);

#[derive(Debug)]
pub struct Framebuffer(GLuint);

impl Framebuffer {
    pub fn new() -> Self {
        let mut fb = 0;
        unsafe {
            glGenFramebuffers(1, &mut fb);
            glBindFramebuffer(GL_FRAMEBUFFER, fb);
        }

        Self(fb)
    }

    pub fn attach<F: Format>(&self, attach: Attachment, tex: &Texture<F>) {
        unsafe {
            glFramebufferTexture(GL_FRAMEBUFFER, attach as _, **tex, 0);
            glDrawBuffers(1, [attach as _].as_ptr());
        }
    }
}

impl Resource for Framebuffer {
    fn bind(&self) {
        log::debug!("binding framebuffer {}", self.0);
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.0);
        }
    }
}

impl Target for Framebuffer {}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        log::debug!("dropping framebuffer {}", self.0);
        unsafe {
            glDeleteFramebuffers(1, &self.0);
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Attachment {
    Color0 = GL_COLOR_ATTACHMENT0,
    Stencil = GL_STENCIL_ATTACHMENT,
}
