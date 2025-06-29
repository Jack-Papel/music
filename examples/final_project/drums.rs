use lazy_static::lazy_static;
use symphoxy::prelude::*;

lazy_static!(
    static ref KICK: NotePitch = C4.octave(-1);
    static ref SNARE: NotePitch = C4;
    static ref HI_HAT: NotePitch = C4.octave(1);
    static ref CRASH: NotePitch = C4.octave(2);
);

pub fn drums_verse() -> Line {
    let kick = C4.octave(-1);
    let snare = C4;

    drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) + sixteenth(kick)) +
    drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) * 2)
}

pub fn drums_prechorus() -> Line {
    let kick = C4.octave(-1);
    let snare = C4;

    drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + eighth(snare))
}

pub fn groovy_drums() -> Line {
    let kick = C4.octave(-1);
    let hi_hat = C4.octave(1);

    drums(sixteenth(kick) + sixteenth(hi_hat) * 2 + (sixteenth(kick) + sixteenth(hi_hat)) * 2 + sixteenth(kick)) * 3 +
    drums(sixteenth(kick) + sixteenth(hi_hat) * 2 + sixteenth(kick) + (sixteenth(kick) + sixteenth(hi_hat)) * 2)
}

pub fn groovy_drums_alt() -> Line {
    let kick = C4.octave(-1);
    let snare = C4;
    let hi_hat = C4.octave(1);

    drums(sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(hi_hat) + sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(kick)) * 3 +
    drums(sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(kick)) * 2
}