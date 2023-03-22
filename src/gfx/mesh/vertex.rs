use {crate::gfx::*, core::mem::size_of};

pub trait Vertex: Sized {
    const NUM: i32 = 0;
    const SIZE: i32 = size_of::<Self>() as _;
    const TYPE: GLenum = GL_FLOAT;
    const NORM: bool = false;

    fn bind() {
        Self::bind_parameters(0, Self::NUM, Self::TYPE, Self::NORM, Self::SIZE, 0);
    }

    fn bind_parameters(idx: u32, num: i32, _type: GLenum, norm: bool, size: i32, stride: i32) {
        unsafe {
            glEnableVertexAttribArray(idx);
            glVertexAttribPointer(idx, num, _type, norm as _, size, stride as _);
        }
    }
}

impl<V1: Vertex, V2: Vertex> Vertex for (V1, V2) {
    fn bind() {
        V1::bind_parameters(0, V1::NUM, V1::TYPE, V1::NORM, Self::SIZE, 0);
        V2::bind_parameters(1, V2::NUM, V2::TYPE, V2::NORM, Self::SIZE, V1::SIZE);
    }
}

impl<const N: usize> Vertex for [f32; N] {
    const NUM: i32 = N as _;
}
