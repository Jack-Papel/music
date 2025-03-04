use crate::note::NotePitch;

pub mod tet12;

pub trait Scale {
    fn get_pitch(&self, degree: isize) -> NotePitch;
}