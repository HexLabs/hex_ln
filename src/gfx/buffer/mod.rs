use {crate::gfx::*, core::mem::size_of, Resource};

pub struct Buffer {
    _type: GLenum,
    buf: GLuint,
    len: usize,
}

impl Buffer {
    pub fn new<Data>(_type: GLenum, usage: Usage, data: &[Data]) -> Self {
        unsafe {
            let mut buf = 0;
            glGenBuffers(1, &mut buf);
            glBindBuffer(_type, buf);

            glBufferData(
                _type,
                (size_of::<Data>() * data.len()) as _,
                data.as_ptr() as _,
                usage as _,
            );

            Self {
                _type,
                buf,
                len: data.len(),
            }
        }
    }

    pub fn update<Data>(&mut self, data: &[Data]) {
        self.bind();
        unsafe {
            glBufferSubData(
                self._type,
                0,
                (size_of::<Data>() * data.len()) as _,
                data.as_ptr() as _,
            );
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Resource for Buffer {
    fn bind(&self) {
        unsafe { glBindBuffer(self._type, self.buf) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, [self.buf].as_ptr());
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Usage {
    StaticDraw = GL_STATIC_DRAW,
    StreamDraw = GL_STREAM_DRAW,
}
