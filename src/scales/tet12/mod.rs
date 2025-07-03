use std::ops::Mul;

/// Musical modes and scale implementations.
///
/// Contains implementations of various musical scales and modes
/// (major, minor, dorian, lydian, etc.) in the 12-tone equal temperament system.
pub mod modes;

pub use modes::*;

use crate::{
    instrument_tools::strings::StringTuning,
    note::{chord::Chord, NotePitch},
};

/// Gets the note name (without octave) for a given pitch.
///
/// Returns the note name in standard Western notation (C, C#, D, D#, E, F, F#, G, G#, A, A#, B)
/// relative to the provided A4 reference pitch.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
/// use symphoxy::scales::tet12::get_note_name;
///
/// let note_name = get_note_name(C4, A4);
/// assert_eq!(note_name, "C");
///
/// let sharp_name = get_note_name(NotePitch::new(277.18), A4); // C#4
/// assert_eq!(sharp_name, "C#");
/// ```
pub fn get_note_name(note: NotePitch, a4: NotePitch) -> String {
    let name = get_note_name_with_octave(note, a4);
    name.trim_end_matches(char::is_numeric).to_string()
}

/// Gets the note name with octave number for a given pitch.
///
/// Returns the note name with octave in standard Western notation (e.g., "C4", "A#5")
/// relative to the provided A4 reference pitch.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
/// use symphoxy::scales::tet12::get_note_name_with_octave;
///
/// let note_name = get_note_name_with_octave(C4, A4);
/// assert_eq!(note_name, "C4");
///
/// let higher_note = get_note_name_with_octave(NotePitch::new(880.0), A4); // A5
/// assert_eq!(higher_note, "A5");
/// ```
pub fn get_note_name_with_octave(note: NotePitch, a4: NotePitch) -> String {
    let c4 = a4.semitone(3).octave(-1);

    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

    let diff = f32::log2(note.0 / c4.0);

    #[expect(clippy::cast_possible_truncation, reason = "log_2 of a non-infinite f32 has at most 7 bits")]
    let (octave_diff, semitone_diff) = (diff.floor() as i16, ((diff * 12.0).round() as i16).rem_euclid(12));

    #[expect(clippy::cast_sign_loss, reason = "semitone_diff is always in range 0..12")]
    let note_name = String::from(note_names[semitone_diff as usize]);

    #[expect(clippy::arithmetic_side_effects, reason = "This is guaranteed to fit in i16.")]
    let octave_number = octave_diff + 4;

    #[expect(clippy::arithmetic_side_effects, reason = "This is a simple string concatenation")]
    let out = note_name + &(octave_number).to_string();

    out
}

#[test]
fn test_get_note_name() {
    let notes = A4.semitones([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    let expected_names = [
        "A4", "A#4", "B4", "C5", "C#5", "D5", "D#5", "E5", "F5", "F#5", "G5", "G#5",
    ];

    for (note, expected_name) in notes.iter().zip(expected_names.iter()) {
        let name = get_note_name_with_octave(*note, A4);
        assert_eq!(name, *expected_name);
    }
}

/// Standard pitch reference - A above middle C at 440 Hz.
///
/// This is the international standard tuning reference pitch.
pub const A4: NotePitch = NotePitch(440.0);
/// Middle C pitch at approximately 261.626 Hz.
///
/// This is a common reference point for musical compositions.
pub const C4: NotePitch = NotePitch(261.626);

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "Willing to accept some precision loss here"
)]
fn get_degree_with_pattern_and_root(degree: isize, root: NotePitch, pattern: [f64; 7]) -> NotePitch {
    #[expect(clippy::arithmetic_side_effects, reason = "Manual overflow checking")]
    let adjusted_degree = if degree > 0 { degree - 1 } else { degree };
    let octave_power = adjusted_degree.div_euclid(7) as f64;

    let mut interval_power = 0.0f64;
    for &step_size in pattern.iter().take(adjusted_degree.rem_euclid(7) as usize) {
        interval_power += step_size / 12.0
    }

    let factor = 2.0f64.powf(octave_power + interval_power);

    let pitch = (root.0 as f64).mul(factor) as f32;

    NotePitch(pitch)
}

/// A trait for 12-tone equal temperament pitch manipulation.
///
/// This trait provides methods for transposing pitches by octaves and semitones
/// within the 12-tone equal temperament system, where each octave is divided
/// into 12 equal semitones.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Octave transposition
/// let c5 = C4.octave(1);   // C4 up one octave = C5
/// let c3 = C4.octave(-1);  // C4 down one octave = C3
///
/// // Semitone transposition  
/// let cs4 = C4.semitone(1);  // C4 up one semitone = C#4
/// let b3 = C4.semitone(-1);  // C4 down one semitone = B3
///
/// // Multiple semitones at once
/// let major_triad_intervals = C4.semitones([0, 4, 7]); // C4, E4, G4
/// ```
pub trait Tet12 {
    /// Transposes the pitch by the specified number of octaves.
    ///
    /// Positive values transpose up, negative values transpose down.
    /// Each octave represents a doubling (or halving) of frequency.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let c5 = C4.octave(1);   // Up one octave
    /// let c2 = C4.octave(-2);  // Down two octaves
    /// ```
    fn octave(&self, change: i32) -> Self;

    /// Transposes the pitch by the specified number of semitones.
    ///
    /// Positive values transpose up, negative values transpose down.
    /// In 12-tone equal temperament, 12 semitones equal one octave.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let cs4 = C4.semitone(1);   // Up one semitone (C# / Db)
    /// let f4 = C4.semitone(5);    // Up five semitones (perfect fourth)
    /// let g4 = C4.semitone(7);    // Up seven semitones (perfect fifth)
    /// ```
    fn semitone(&self, change: i16) -> Self;

    /// Get several notes from this note by specifying a list of semitone offsets.
    fn semitones<const N: usize>(&self, changes: [i16; N]) -> [Self; N]
    where
        Self: Sized + Clone + Copy,
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
