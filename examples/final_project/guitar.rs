#![cfg_attr(rustfmt, rustfmt_skip)]

use lazy_static::lazy_static;
use symphoxy::{instrument_tools::strings::{Frets, GuitarTuning}, note::chord::Chord, prelude::*};

lazy_static!(
    static ref TUNING: GuitarTuning = GuitarTuning::DEFAULT_GUITAR_TUNING.semitone(-1);

    static ref GUITAR_I: Chord = TUNING.get_chord(&Frets::new_full([0, 2, 2, 2, 0, 0]));
    static ref GUITAR_VI_M: Chord = TUNING.get_chord(&Frets::new_full([2, 2, 2, 4, 4, 2]));
    static ref GUITAR_II_DIM_7: Chord = TUNING.get_chord(&Frets::new_full([1, 3, 1, 2, 2, 1]));
    static ref GUITAR_V_7_SUS_4: Chord = TUNING.get_chord(&Frets::new_full([0, 0, 2, 0, 0, 0]));
);

fn groove(note: NotePitch) -> Line {
    electric_guitar(eighth(REST) + eighth(note) + (sixteenth(REST) + sixteenth(note)) * 2)
}

fn chord_strike(note: NotePitch) -> Line {
    electric_guitar(eighth(REST) + sixteenth(note) * 2)
}

fn epic_chord_strike(note: NotePitch) -> Line {
    electric_guitar(quarter(note)).into()
}

pub fn dings() -> Piece {
    GUITAR_I.strike(chord_strike) * 4 +
    GUITAR_VI_M.strike(chord_strike) * 2 +
    GUITAR_II_DIM_7.strike(chord_strike) +
    GUITAR_V_7_SUS_4.strike(chord_strike)
}

pub fn dings_prechorus() -> Piece {
    GUITAR_I.strike(chord_strike) * 4 +
    GUITAR_VI_M.strike(chord_strike) * 2 +
    GUITAR_II_DIM_7.strike(epic_chord_strike) + 
    GUITAR_V_7_SUS_4.strike(epic_chord_strike)
}

pub fn groovy_dings() -> Piece {
    GUITAR_I.strike(groove) + 
    GUITAR_VI_M.strike(groove) +
    GUITAR_II_DIM_7.strike(groove) + 
    GUITAR_V_7_SUS_4.strike(groove)
}

pub fn groovy_dings_prechorus() -> Piece {
    GUITAR_I.strike(groove) * 2 + 
    GUITAR_VI_M.strike(groove) +
    GUITAR_II_DIM_7.strike(epic_chord_strike) + 
    GUITAR_V_7_SUS_4.strike(epic_chord_strike)
}