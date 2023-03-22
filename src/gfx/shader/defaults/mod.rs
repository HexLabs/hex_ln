macro_rules! shader_src {
    ($src:literal) => {
        concat!(include_str!($src), "\0")
    };
}

pub const POS2D: &str = shader_src!("pos2d.vert");
pub const POS2D_TEX2D: &str = shader_src!("pos2d_tex2d.vert");
pub const POS3D: &str = shader_src!("pos3d.vert");
pub const POS2D_RGB: &str = shader_src!("pos2d_rgb.vert");
pub const POS3D_RGB: &str = shader_src!("pos3d_rgb.vert");
pub const WHITE: &str = shader_src!("white.frag");
pub const RGB: &str = shader_src!("rgb.frag");
pub const TEX2D: &str = shader_src!("tex2d.frag");
