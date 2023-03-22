use {crate::gfx::*, core::marker::PhantomData};

pub type TextureRgb = Texture<[f32; 3]>;
pub type TextureRgba = Texture<[f32; 4]>;
pub type TextureStencil = Texture<i32>;

pub trait Format {
    const GL_FORMAT: u32;
}

impl Format for [f32; 3] {
    const GL_FORMAT: u32 = GL_RGB;
}

impl Format for [f32; 4] {
    const GL_FORMAT: u32 = GL_RGBA;
}

impl Format for i32 {
    const GL_FORMAT: u32 = GL_STENCIL_INDEX;
}

pub const TEX_2D: u32 = GL_TEXTURE_2D;

#[derive(Debug)]
pub struct Texture<F: Format> {
    id: GLuint,
    target: GLuint,
    format: PhantomData<F>,
}

impl<F: Format> Texture<F> {
    pub fn new(target: u32, [w, h]: [i32; 2]) -> Self {
        let mut id = 0;
        unsafe {
            glGenTextures(1, &mut id);
            glBindTexture(target as _, id);
            glTexImage2D(
                target as _,
                0,
                F::GL_FORMAT as _,
                w,
                h,
                0,
                F::GL_FORMAT as _,
                GL_UNSIGNED_BYTE,
                core::ptr::null(),
            );
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as _);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as _);
        }

        Self {
            id,
            target,
            format: PhantomData,
        }
    }
}

impl<F: Format> Resource for Texture<F> {
    fn bind(&self) {
        log::debug!("binding texture {}", self.id);
        unsafe {
            glBindTexture(self.target as _, self.id);
        }
    }
}

impl<F: Format> core::ops::Deref for Texture<F> {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<F: Format> Drop for Texture<F> {
    fn drop(&mut self) {
        log::debug!("dropping texture {}", self.id);
        unsafe {
            glDeleteTextures(1, &self.id);
        }
    }
}
