pub mod color;
pub mod ray;
pub mod vector;

pub fn gen_2d_range(from: i32, to: i32) -> impl Iterator<Item = (i32, i32)> {
    (from..to).flat_map(move |a| (from..to).map(move |b| (a, b)))
}
