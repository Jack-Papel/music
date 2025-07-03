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
    ($name:ident, $pattern:expr, $doc:expr) => {
        #[doc = $doc]
        pub struct $name(pub NotePitch);

        impl Scale for $name {
            fn get_degree(&self, degree: isize) -> NotePitch {
                get_degree_with_pattern_and_root(degree, self.0, $pattern)
            }
        }
    };
}

implement_scale!(
    LydianScale,
    scale_pattern![w w w h w w h],
    "Lydian mode - a major-type scale with a raised 4th degree, creating a bright, dreamy sound."
);
implement_scale!(
    MajorScale,
    scale_pattern![w w h w w w h],
    "Major scale - the most common Western scale, providing a happy, bright sound. Also known as Ionian mode."
);
implement_scale!(
    MixolydianScale,
    scale_pattern![w w h w w h w],
    "Mixolydian mode - a major-type scale with a flattened 7th degree."
);
implement_scale!(
    DorianScale,
    scale_pattern![w h w w w h w],
    "Dorian mode - a minor-type scale with a raised 6th degree."
);
implement_scale!(
    MinorScale,
    scale_pattern![w h w w h w w],
    "Natural minor scale - provides a sad, melancholic sound. Also known as Aeolian mode."
);
implement_scale!(
    PhrygianScale,
    scale_pattern![h w w w h w w],
    "Phrygian mode - a minor-type scale with a flattened 2nd degree."
);
implement_scale!(
    LocrianScale,
    scale_pattern![h w w h w w w],
    "Locrian mode - a diminished-type scale with both flattened 2nd and 5th degrees."
);

pub use MajorScale as IonianScale;
pub use MinorScale as AeolianScale;
