mod vertex;

use {
    crate::gfx::{
        buffer::{Buffer, Usage},
        Resource, *,
    },
    core::marker::PhantomData,
    vertex::Vertex,
};

pub struct Mesh<V: Vertex> {
    vao: GLuint,
    vertices: Buffer,
    topology: Topology,
    _data: PhantomData<V>,
}

impl<V: Vertex> Mesh<V> {
    pub fn new(verts: &[V], usage: Usage, topology: Topology) -> Self {
        let mut vao = 0;
        unsafe {
            glGenVertexArrays(1, &mut vao);
            glBindVertexArray(vao);
        }

        match &topology {
            Topology::TriIndexed(indices) => {
                indices.bind();
            }
            _ => {}
        }

        let vertices = Buffer::new(GL_ARRAY_BUFFER, usage, verts);
        V::bind();

        Self {
            vao,
            vertices,
            topology,
            _data: PhantomData,
        }
    }

    pub fn static_draw(verts: &[V], topology: Topology) -> Self {
        Self::new(verts, Usage::StaticDraw, topology)
    }

    pub fn update(&mut self, verts: &[V]) {
        self.vertices.update(verts);
    }

    pub fn stencil(&self) {
        unsafe {
            glEnable(GL_STENCIL_TEST);
            glStencilMask(0xFF);
            glColorMask(0, 0, 0, 0);
            glStencilFunc(GL_ALWAYS, 1, 0xFF);
            glClearStencil(0);
            glClear(GL_STENCIL_BUFFER_BIT);
            glStencilOp(GL_INVERT, GL_INVERT, GL_INVERT);

            self.draw();

            glStencilFunc(GL_NOTEQUAL, 0, 0xFF);
            glStencilOp(GL_KEEP, GL_KEEP, GL_KEEP);
            glColorMask(0xFF, 0xFF, 0xFF, 0xFF);
            glStencilMask(0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.bind();
            match &self.topology {
                Topology::TriIndexed(indices) => {
                    glDrawElements(
                        GL_TRIANGLES,
                        indices.len() as _,
                        GL_UNSIGNED_BYTE,
                        core::ptr::null(),
                    );
                }

                Topology::Lines => {
                    glDrawArrays(GL_LINE_STRIP, 0, self.vertices.len() as _);
                }

                Topology::Points => {
                    glDrawArrays(GL_POINTS, 0, self.vertices.len() as _);
                }

                Topology::TriFan => {
                    glDrawArrays(GL_TRIANGLE_FAN, 0, self.vertices.len() as _);
                }

                Topology::TriStrip => {
                    glDrawArrays(GL_TRIANGLE_STRIP, 0, self.vertices.len() as _);
                }
            }
        }
    }
}

impl<V: Vertex> Resource for Mesh<V> {
    fn bind(&self) {
        unsafe { glBindVertexArray(self.vao) }
    }
}

pub enum Topology {
    Points,
    Lines,
    TriFan,
    TriStrip,
    TriIndexed(Buffer),
}

impl Topology {
    pub fn from_indices(idx: &[u8], usage: Usage) -> Self {
        Self::TriIndexed(Buffer::new(GL_ELEMENT_ARRAY_BUFFER, usage, idx))
    }
}
