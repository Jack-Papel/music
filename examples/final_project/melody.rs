#![cfg_attr(rustfmt, rustfmt_skip)]

use symphoxy::prelude::*;

pub fn im_sad_your_back() -> Line {
    let c5 = C4.octave(1);
    piano(eighth(REST) + eighth(c5) + dotted(eighth)(c5.semitone(-2)) + sixteenth(REST)) +
    piano(eighth(REST) + eighth(c5.semitone(-2)) + dotted(eighth)(c5.semitone(-4)) + sixteenth(REST))
}

pub fn and_i_need_a_big_mac() -> Line {
    let c5 = C4.octave(1);
    piano(dotted(eighth)(REST) + sixteenth(c5.semitone(-9)) + eighth(c5.semitone(-2)) + sixteenth(c5.semitone(-2)) + sixteenth(c5)) +
    piano(eighth(c5.semitone(-2)) + eighth(c5.semitone(-4)) + quarter(REST))
}

pub fn tired_of_being_alone() -> Line {
    let c5 = C4.octave(1);
    -!piano(sixteenth(c5.semitone(-2)) + sixteenth(c5) + sixteenth(c5.semitone(1))) +
    piano(sixteenth(c5.semitone(1)) + sixteenth(c5) + sixteenth(c5.semitone(-2)) + eighth(c5.semitone(-4)) + sixteenth(c5.semitone(-9)) + sixteenth(c5.semitone(-2)) + sixteenth(c5.semitone(-4)) + eighth(c5.semitone(-4)))
}