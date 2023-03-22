extern crate alloc;
use {alloc::vec::Vec, core::intrinsics::powf32};

pub type Matrix<const M: usize, const N: usize> = [[f32; N]; M];

pub fn ortho([left, bottom]: [f32; 2], [right, top]: [f32; 2]) -> Matrix<4, 4> {
    [
        [2.0 / (right - left), 0.0, 0.0, 0.0],
        [0.0, 2.0 / (top - bottom), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [
            -(right + left) / (right - left),
            -(top + bottom) / (top - bottom),
            0.0,
            1.0,
        ],
    ]
}

pub type Spline = Vec<Bezier>;

pub trait Subdivide {
    fn subdivide(&self, n: usize) -> Vec<[f32; 2]>;
}

impl Subdivide for Spline {
    fn subdivide(&self, n: usize) -> Vec<[f32; 2]> {
        self.iter().flat_map(|bezier| bezier.subdivide(n)).collect()
    }
}

#[derive(Debug)]
pub struct Bezier(Vec<[f32; 2]>);

impl From<&[[f32; 2]]> for Bezier {
    fn from(src: &[[f32; 2]]) -> Self {
        Bezier(src.into())
    }
}

impl Subdivide for Bezier {
    fn subdivide(&self, n: usize) -> Vec<[f32; 2]> {
        (0..=n)
            .map(|x| {
                let t = x as f32 / n as f32;
                let mut point = [0.0, 0.0];
                let k = self.0.len() - 1;

                for v in 0..=k {
                    let b = factorial(k) as f32 / (factorial(v) * factorial(k - v)) as f32
                        * powf(t, v as f32)
                        * powf(1.0 - t, (k - v) as f32);

                    point[0] += b * self.0[v][0];
                    point[1] += b * self.0[v][1];
                }

                point
            })
            .collect()
    }
}

fn powf(a: f32, b: f32) -> f32 {
    unsafe { powf32(a, b) }
}

fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n,
    }
}
