use crate::{scales::tet12::get_degree_with_pattern_and_root, NotePitch, Scale};

macro_rules! scale_pattern {
    [$($steps:literal,)* w $($etc:tt)*] => {
        scale_pattern![$($steps,)* 2.0, $($etc)*]
    };
    [$($steps:literal,)* h $($etc:tt)*] => {
        scale_pattern![$($steps,)* 1.0, $($etc)*]
    };
    [$($steps:literal,)*] => {[$($steps,)*]};
}

macro_rules! implement_scale {
    ($name:ident, $pattern:expr) => {
        pub struct $name(pub NotePitch);

        impl Scale for $name {
            fn get_degree(&self, degree: isize) -> NotePitch {
                get_degree_with_pattern_and_root(degree, self.0, $pattern)
            }
        }
    };
}

implement_scale!(LydianScale, scale_pattern![w w w h w w h]);
implement_scale!(MajorScale, scale_pattern![w w h w w w h]);
implement_scale!(MixolydianScale, scale_pattern![w w h w w h w]);
implement_scale!(DorianScale, scale_pattern![w h w w w h w]);
implement_scale!(MinorScale, scale_pattern![w h w w h w w]);
implement_scale!(PhrygianScale, scale_pattern![h w w w h w w]);
implement_scale!(LocrianScale, scale_pattern![h w w h w w w]);

pub use MajorScale as IonianScale;
pub use MinorScale as AeolianScale;