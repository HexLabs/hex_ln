use {
    crate::{
        gfx::{
            buffer::Usage,
            framebuffer::{Attachment, Framebuffer},
            mesh::{Mesh, Topology},
            program::Program,
            shader::{POS2D, WHITE},
            texture::{Texture, TEX_2D},
            Resource, Target,
        },
        math::Spline,
        mem::vec::Vec,
    },
    ttf_parser::{Face, FaceParsingError, OutlineBuilder, Rect},
};

const PIXELS_PER_EM: f32 = 16.0;

#[derive(Debug)]
pub struct Font {
    glyphs: Vec<Option<Glyph>>,
    pub pixels_per_unit: f32,
    pub line_height: i16,
}

impl Font {
    pub fn load(file: &[u8]) -> Result<Self, FaceParsingError> {
        let mut glyphs = Vec::with_capacity(128);

        let face = Face::parse(file, 0)?;
        let builder = GlyphBuilder::new(&face);
        for ch in 0..128u8 {
            let glyph = builder.glyph(ch as char);
            glyphs.push(glyph);
        }

        Ok(Self {
            glyphs,
            pixels_per_unit: PIXELS_PER_EM / face.units_per_em() as f32,
            line_height: face.height(),
        })
    }

    pub fn get(&self, idx: u8) -> Option<&Glyph> {
        self.glyphs[idx as usize].as_ref()
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::load(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/ttf/Hack-Regular.ttf"
        )))
        .expect("failed to load default font")
    }
}

#[derive(Debug)]
pub struct Glyph {
    pub tex: Option<Texture<[f32; 4]>>,
    pub size: [i32; 2],
    pub bearing: [i32; 2],
    pub h_advance: u16,
}

struct GlyphBuilder<'a> {
    face: &'a Face<'a>,
    stencil: Program,
}

impl<'a> GlyphBuilder<'a> {
    fn new(face: &'a Face<'a>) -> Self {
        Self {
            face,
            stencil: Program::new(POS2D, WHITE),
        }
    }

    fn glyph(&self, ch: char) -> Option<Glyph> {
        self.face.glyph_index(ch).map_or_else(
            || {
                log::debug!("skipping unprintable character {}", ch.escape_default());
                None
            },
            |idx| {
                log::debug!("constructing glyph for '{}'", ch.escape_default());

                let mut outline = SplineBuilder::new();
                let (tex, size) = self.face.outline_glyph(idx, &mut outline).map_or_else(
                    || {
                        log::debug!("no outline found for '{}'", ch.escape_default());
                        (None, [0, 0])
                    },
                    |Rect {
                         x_min,
                         x_max,
                         y_min,
                         y_max,
                     }| {
                        let size = [(x_max - x_min) as i32, (y_max - y_min) as i32];

                        let segments = outline.build();
                        log::debug!(
                            "'{}' has outline with size {:?} and {} segments",
                            ch.escape_default(),
                            size,
                            segments.len(),
                        );

                        let verts: Vec<[f32; 2]> = segments
                            .iter()
                            .flat_map(|spline| {
                                spline.iter().flat_map(|bezier| {
                                    bezier.subdivide(10).map(|point| {
                                        [
                                            (2.0 * (point[0] - x_min as f32) / size[0] as f32)
                                                - 1.0,
                                            (2.0 * (point[1] - y_min as f32) / size[1] as f32)
                                                - 1.0,
                                        ]
                                    })
                                })
                            })
                            .collect();

                        let glyph = Mesh::new(&verts, Usage::StaticDraw, Topology::TriFan);
                        let quad = Mesh::new(
                            &[[-1.0, 1.0], [1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]],
                            Usage::StaticDraw,
                            Topology::TriStrip,
                        );

                        let tex: Texture<[f32; 4]> = Texture::new(TEX_2D, size);
                        let stencil: Texture<i32> = Texture::new(TEX_2D, size);

                        let fb = Framebuffer::new();
                        fb.attach(Attachment::Color0, &tex);
                        fb.attach(Attachment::Stencil, &stencil);

                        fb.bind();
                        fb.viewport([0, 0], size);
                        fb.clear_color([0.0, 0.0, 0.0, 0.0]);

                        self.stencil.bind();
                        stencil.bind();
                        glyph.stencil();
                        quad.draw();

                        (Some(tex), size)
                    },
                );

                let h_advance = self.face.glyph_hor_advance(idx).unwrap_or(0);
                let bearing = [
                    self.face.glyph_hor_side_bearing(idx).unwrap_or(0) as i32,
                    self.face.glyph_ver_side_bearing(idx).unwrap_or(0) as i32,
                ];

                Some(Glyph {
                    tex,
                    size,
                    bearing,
                    h_advance,
                })
            },
        )
    }
}

pub struct SplineBuilder {
    splines: Vec<Spline>,
    head: [f32; 2],
}

impl SplineBuilder {
    pub fn new() -> Self {
        Self {
            splines: Vec::new(),
            head: [0.0, 0.0],
        }
    }

    pub fn build(self) -> Vec<Spline> {
        self.splines
    }
}

impl OutlineBuilder for SplineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.splines.push(Spline::with_capacity(1));
        self.head = [x, y];
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let idx = self.splines.len() - 1;
        self.splines[idx].push([self.head, [x, y]].into());
        self.head = [x, y];
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let idx = self.splines.len() - 1;
        self.splines[idx].push([self.head, [x1, y1], [x, y]].into());
        self.head = [x, y];
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let idx = self.splines.len() - 1;
        self.splines[idx].push([self.head, [x1, y1], [x2, y2], [x, y]].into());
        self.head = [x, y];
    }

    fn close(&mut self) {}
}
