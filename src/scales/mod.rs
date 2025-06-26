use crate::{note::NotePitch, scales::tet12::A4};

pub mod tet12;

pub use tet12::MajorScale;

pub trait Scale {
    fn get_degree(&self, degree: isize) -> NotePitch;
    fn get_degrees<const N: usize>(&self, degrees: [isize; N]) -> [NotePitch; N] {
        let mut out = [A4; N];
        for (idx, &degree) in degrees.iter().enumerate() {
            out[idx] = self.get_degree(degree);
        }
        out
    }
}