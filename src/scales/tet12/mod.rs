use std::ops::Mul;

pub mod modes;

pub use modes::*;

use crate::{instrument_tools::strings::StringTuning, note::{chord::Chord, NotePitch}};

pub fn get_note_name(note: NotePitch, a4: NotePitch) -> String {
    let name = get_note_name_with_octave(note, a4);
    name.trim_end_matches(char::is_numeric).to_string()
}

pub fn get_note_name_with_octave(note: NotePitch, a4: NotePitch) -> String {
    let c4 = a4.semitone(3).octave(-1);

    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

    let diff = f32::log2(note.0 / c4.0);
    #[expect(clippy::cast_possible_truncation, reason = "log_2 of a non-infinite f32 has at most 7 bits")]
    let (octave_diff, semitone_diff) = (diff.round() as i16, ((diff * 12.0).round() as i16).rem_euclid(12));

    #[expect(clippy::cast_sign_loss, reason = "semitone_diff is always in range 0..12")]
    let note_name = String::from(note_names[semitone_diff as usize]);

    #[expect(clippy::arithmetic_side_effects, reason = "This is guaranteed to fit in i16.")]
    let octave_number = octave_diff + 4;

    #[expect(clippy::arithmetic_side_effects, reason = "This is a simple string concatenation")]
    let out = note_name + &(octave_number).to_string();

    out
}

pub const A4: NotePitch = NotePitch(440.0);
pub const C4: NotePitch = NotePitch(261.626);

#[expect(clippy::cast_possible_truncation, clippy::cast_precision_loss, reason = "Willing to accept some precision loss here")]
fn get_degree_with_pattern_and_root(degree: isize, root: NotePitch, pattern: [f64; 7]) -> NotePitch {
    #[expect(clippy::arithmetic_side_effects, reason = "Manual overflow checking")]
    let adjusted_degree = if degree > 0 {
        degree - 1
    } else {
        degree
    };
    let octave_power = adjusted_degree.div_euclid(7) as f64;

    let mut interval_power = 0.0f64;
    for &step_size in pattern.iter().take(adjusted_degree.rem_euclid(7) as usize) {
        interval_power += step_size / 12.0
    }

    let factor = 2.0f64.powf(octave_power + interval_power);

    let pitch = (root.0 as f64).mul(factor) as f32;

    NotePitch(pitch)
}


pub trait Tet12 {
    fn octave(&self, change: i32) -> Self;
    fn semitone(&self, change: i16) -> Self;
    /// Get several notes from this note by specifying a list of semitone offsets.
    fn semitones<const N: usize>(&self, changes: [i16; N]) -> [Self; N]
        where Self: Sized + Clone + Copy
    {
        let mut result = [*self; N];
        for (i, &change) in changes.iter().enumerate() {
            result[i] = result[i].semitone(change);
        }
        result
    }
}


impl Tet12 for NotePitch {
    fn octave(&self, change: i32) -> Self {
        Self(self.0 * 2.0f32.powi(change))
    }

    fn semitone(&self, change: i16) -> Self {
        Self(self.0 * 2.0f32.powf(change as f32 / 12.0))
    }
}

impl<const N: usize> Tet12 for StringTuning<N> {
    fn octave(&self, change: i32) -> Self {
        StringTuning(self.0.map(|note| note.octave(change)))
    }

    fn semitone(&self, change: i16) -> Self {
        StringTuning(self.0.map(|note| note.semitone(change)))
    }
}

impl Tet12 for Chord {
    fn octave(&self, change: i32) -> Self {
        Chord::new(self.0.iter().map(|&note| note.octave(change)))
    }

    fn semitone(&self, change: i16) -> Self {
        Chord::new(self.0.iter().map(|&note| note.semitone(change)))
    }
}