pub mod buffer;
pub mod framebuffer;
pub mod mesh;
pub mod program;
pub mod shader;
pub mod texture;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl.rs"));
}

use {gl::*,  crate::math::Matrix};
pub use framebuffer::SWAP_CHAIN;

pub trait Resource {
    fn bind(&self);
}

pub trait Target: Resource {
    fn clear_color(&self, [r, g, b, a]: [f32; 4]) {
        unsafe {
            glClearColor(r, g, b, a);
            glClear(GL_COLOR_BUFFER_BIT);
        }
    }

    fn clear_stencil(&self, clear: i32) {
        unsafe {
            glClearStencil(clear);
            glClear(GL_STENCIL_BUFFER_BIT);
        }
    }

    fn viewport(&self, [x, y]: [i32; 2], [w, h]: [i32; 2]) {
        unsafe {
            glViewport(x, y, w, h);
        }
    }
}

pub trait Uniform {
    fn bind(&self, location: i32);
}

impl Uniform for Matrix<4, 4> {
    fn bind(&self, location: i32) {
        unsafe {
            glUniformMatrix4fv(location, 1, GL_FALSE as _, self.as_ptr() as _);
        }
    }
}
