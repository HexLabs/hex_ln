use crate::{
    gfx::{
        buffer::Usage,
        framebuffer::{Attachment, Framebuffer},
        mesh::{Mesh, Topology},
        program::Program,
        texture::{Texture, TextureRgba, TEX_2D},
        Resource, Target, Uniform,
    },
    gui::font::{Font, Glyph},
    math::ortho,
    mem::vec::Vec,
};

static TEXT_VERT: &str = concat!(
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/shaders/text.vert"
    )),
    "\0"
);
static TEXT_FRAG: &str = concat!(
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/shaders/text.frag"
    )),
    "\0"
);

#[derive(Debug)]
pub struct TextBox {
    text: Vec<u8>,
    tex: TextureRgba,
    buf: Framebuffer,
}

impl TextBox {
    pub fn new(size: [i32; 2]) -> Self {
        let tex = Texture::new(TEX_2D, size);
        let buf = Framebuffer::new();
        buf.attach(Attachment::Color0, &tex);

        Self {
            text: Vec::with_capacity(1),
            tex,
            buf,
        }
    }

    pub fn draw(&self, [w, h]: [i32; 2], font: &Font, em: f32) {
        log::debug!("rendering {:?}", self.text);
        let program = Program::new(TEXT_VERT, TEXT_FRAG);
        program.bind();

        self.buf.bind();
        self.buf.viewport([0, 0], [w, h]);
        self.buf.clear_color([0.0, 0.0, 0.0, 1.0]);

        // Build an orthographic projection matrix
        ortho([0.0, 0.0], [w as f32, h as f32]).bind(0);

        // Position the cursor for the first character
        let scale = em * font.pixels_per_unit;
        let mut y = h as f32 - font.line_height as f32 * scale;
        let mut x = 0.0;
        for byte in self.text.iter() {
            match byte {
                b'\n' => {
                    // Move cursor to beginning of next line
                    y -= font.line_height as f32 * scale;
                    x = 0.0;
                }

                b'\t' => {
                    // Advance by 4 spaces
                    let &Glyph { h_advance, .. } =
                        font.get(b' ').expect("failed to look up h_advance for ' '");
                    x += 4.0 * h_advance as f32 * scale;
                }

                &ch => {
                    // Draw the character if it has an outline
                    let glyph = font.get(ch).expect("character not found");
                    if let Some(tex) = &glyph.tex {
                        let [w, h] = [glyph.size[0] as f32 * scale, glyph.size[1] as f32 * scale];
                        let [dx, dy] = [
                            glyph.bearing[0] as f32 * scale,
                            glyph.bearing[1] as f32 * scale,
                        ];

                        let left = x + dx;
                        let right = left + w;
                        let bottom = y + dy;
                        let top = bottom + h;

                        log::debug!(
                            "rendering {} with bottom left ({}, {}) and top right ({}, {})",
                            (ch as char).escape_default(),
                            left,
                            bottom,
                            right,
                            top
                        );

                        tex.bind();
                        Mesh::new(
                            &[
                                ([left, top], [0.0, 1.0]),
                                ([right, top], [1.0, 1.0]),
                                ([left, bottom], [0.0, 0.0]),
                                ([right, bottom], [1.0, 0.0]),
                            ],
                            Usage::StaticDraw,
                            Topology::TriStrip,
                        )
                        .draw();
                    }

                    x += glyph.h_advance as f32 * scale;
                }
            }
        }
    }

    pub fn update(&mut self, text: &str) {
        self.text = text.as_bytes().into();
    }

    pub fn view(&self) -> &TextureRgba {
        &self.tex
    }
}
