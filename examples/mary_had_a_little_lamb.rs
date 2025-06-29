use symphoxy::prelude::*;
use symphoxy::InteractiveTui;

fn mary_had_a_little_lamb() -> impl Into<Piece> {
    let c_major = symphoxy::scales::tet12::IonianScale(C4);
    let [c4, d4, e4, g4] = c_major.get_degrees([1, 2, 3, 5]);
    
    piano(
        quarter(e4) + quarter(d4) + quarter(c4) + quarter(d4) +
        quarter(e4) * 3 + quarter(REST) +
        quarter(d4) * 3 + quarter(REST) +
        quarter(e4) + quarter(g4) * 2 + quarter(REST) +
        quarter(e4) + quarter(d4) + quarter(c4) + quarter(d4) +
        quarter(e4) * 4 + quarter(d4) * 2 +
        quarter(e4) + quarter(d4) + quarter(c4) + quarter(REST)
        + quarter(c4.octave(1))
    )
}

fn main() {
    let song: Piece = mary_had_a_little_lamb().into();
    InteractiveTui::start(song);
}
