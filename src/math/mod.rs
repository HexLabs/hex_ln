use {crate::mem::vec::Vec, core::intrinsics::powf32};

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
pub struct Bezier(Vec<[f32; 2]>);

impl Bezier {
    pub fn subdivide(&self, n: usize) -> impl Iterator<Item = [f32; 2]> + '_ {
        log::debug!("subdividing bezier path {:?}", self.0);
        (0..=n).map(move |x| {
            let t = x as f32 / n as f32;
            let n = self.0.len() - 1;

            self.0
                .iter()
                .enumerate()
                .fold([0.0, 0.0], |sum, (i, point)| {
                    let b = factorial(n) as f32 / (factorial(i) * factorial(n - i)) as f32
                        * powf(t, i as f32)
                        * powf(1.0 - t, (n - i) as f32);

                    [sum[0] + b * point[0], sum[1] + b * point[1]]
                })
        })
    }
}

impl<T: Into<Vec<[f32; 2]>>> From<T> for Bezier {
    fn from(from: T) -> Self {
        Self(from.into())
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
