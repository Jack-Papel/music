/// Chord types and harmonic functionality.
///
/// Contains the `Chord` type for representing groups of pitches played simultaneously.
pub mod chord;
mod length;
mod timbre;

pub use length::*;
pub use timbre::*;

use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use crate::{Line, Piece, A4};

/// Represents a musical note with duration, pitch/rest, and timbre
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Create a quarter note C4 with piano timbre
/// let note = piano(quarter(NotePitch(261.626)));
/// ```
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Note(pub NoteLength, pub NoteKind);

impl Note {
    /// Creates a new note with the specified volume level.
    ///
    /// For pitched notes, this sets the volume parameter. For rests,
    /// this has no effect since rests don't produce sound.
    ///
    /// # Parameters
    /// - `volume`: Volume level (0.0 = silent, 1.0 = full volume, can exceed 1.0)
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let note = piano(quarter(C4));
    /// let quiet_note = note.volume(0.5);  // Half volume
    /// let loud_note = note.volume(2.0);   // Double volume
    ///
    /// let rest = quarter(REST);
    /// let same_rest = rest.volume(0.5);   // No effect on rests
    ///
    /// assert_eq!(rest, same_rest); // Rests remain unchanged
    /// ```
    /// When changing the volume of a note, it is important to note that
    /// this is not a multiplication of the note's volume, but rather a setting
    /// of the volume parameter in the `NoteKind::Pitched` variant.
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let note = piano(quarter(A4).volume(0.5));
    /// let loud_note = note.volume(2.0); // Sets volume to 2.0
    ///
    /// assert!(matches!(loud_note.1, NoteKind::Pitched { volume: 2.0, .. })); // Volume is now 2.0, not 1.0
    /// ```
    pub fn volume(&self, volume: f32) -> Note {
        let new_note_kind = match self.1 {
            NoteKind::Pitched { pitch, timbre, .. } => NoteKind::Pitched { pitch, timbre, volume },
            NoteKind::Rest => NoteKind::Rest,
        };

        Note(self.0, new_note_kind)
    }
}

impl Add<Note> for Note {
    type Output = Line;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic Implementation")]
    fn add(self, rhs: Note) -> Self::Output {
        let self_line: Line = self.into();
        let rhs_line: Line = rhs.into();
        self_line + rhs_line
    }
}

impl Add<Line> for Note {
    type Output = Line;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic Implementation")]
    fn add(self, rhs: Line) -> Self::Output {
        let self_line: Line = self.into();
        self_line + rhs
    }
}

impl Mul<usize> for Note {
    type Output = Line;

    fn mul(self, rhs: usize) -> Self::Output {
        Line::from((0..rhs).map(|_| self).collect::<Vec<_>>())
    }
}

impl Mul<Note> for Note {
    type Output = Piece;

    fn mul(self, rhs: Note) -> Self::Output {
        let self_line: Line = self.into();
        let rhs_line: Line = rhs.into();

        Piece(vec![self_line, rhs_line])
    }
}

/// Represents different kinds of musical notes - either a pitched sound or a rest (silence).
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Manually create a pitched note at 440Hz (A4) with piano timbre
/// // Usually, you would just do `piano(quarter(A4))`
/// let a4_note = NoteKind::Pitched {
///     pitch: NotePitch(440.0),
///     timbre: Timbre::Piano,
///     volume: 1.0
/// };
///
/// // Create a rest
/// let rest = NoteKind::Rest;
///
/// // Or use the constant
/// let rest2 = REST;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NoteKind {
    /// A rest - produces no sound for the duration specified
    #[default]
    Rest,
    /// A pitched note with frequency, timbre, and volume
    Pitched {
        /// The fundamental frequency of the note in Hz
        pitch: NotePitch,
        /// The sound characteristics (sine wave, piano, guitar, etc.)
        timbre: Timbre,
        /// Volume level (0.0 = silent, 1.0 = full volume, can exceed 1.0)
        volume: f32,
    },
}

impl From<NotePitch> for NoteKind {
    fn from(value: NotePitch) -> Self {
        NoteKind::Pitched {
            pitch: value,
            timbre: Timbre::default(),
            volume: 1.0,
        }
    }
}

/// Represents a musical pitch as a frequency in Hz.
///
/// This is a newtype wrapper around `f32` that represents the fundamental frequency
/// of a musical note. Common frequencies include A4 = 440Hz, C4 = 261.626Hz.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Create pitches from frequencies
/// let a4 = NotePitch::new(440.0);
/// let c4 = NotePitch::new(261.626);
///
/// // Convert from f32
/// let a5: NotePitch = 880.0.into();
///
/// // Get frequency back
/// assert_eq!(a4.frequency(), 440.0);
/// ```
///
/// # Note
/// Since this wraps an f32, it does not implement `Eq` or `Hash` due to
/// floating-point comparison issues. Use `PartialEq` for comparisons.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct NotePitch(pub f32);

impl Debug for NotePitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::scales::tet12::get_note_name_with_octave(*self, A4))
    }
}

impl From<f32> for NotePitch {
    fn from(frequency: f32) -> Self {
        NotePitch(frequency)
    }
}

impl From<NotePitch> for f32 {
    fn from(pitch: NotePitch) -> Self {
        pitch.0
    }
}

impl NotePitch {
    /// Creates a new `NotePitch` from a frequency in Hz.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let a4 = NotePitch::new(440.0);
    /// let c4 = NotePitch::new(261.626);
    /// ```
    pub fn new(frequency: f32) -> Self {
        NotePitch(frequency)
    }

    /// Gets the frequency of this pitch in Hz.
    pub fn frequency(&self) -> f32 {
        self.0
    }
}

impl From<NoteLength> for u16 {
    fn from(length: NoteLength) -> Self {
        length.0
    }
}

impl From<u16> for NoteLength {
    fn from(length: u16) -> Self {
        NoteLength(length)
    }
}

// Display implementations for better debugging
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for NotePitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}: {:.2}Hz",
            crate::scales::tet12::get_note_name_with_octave(*self, A4),
            self.0
        )
    }
}

impl Display for NoteLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} beats", self.0)
    }
}

impl NoteLength {
    /// Creates a new `NoteLength` from a duration value.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let quarter_len = NoteLength::new(4);
    /// let half_len = NoteLength::new(8);
    ///
    /// // Check that they work as expected
    /// let length = quarter_len.clone();
    /// assert_eq!(length.duration(), 4);
    /// ```
    pub fn new(duration: u16) -> Self {
        NoteLength(duration)
    }

    /// Gets the duration value of this note length.
    pub fn duration(&self) -> u16 {
        self.0
    }
}

/// A constant representing a musical rest (silence).
///
/// This is equivalent to `NoteKind::Rest` but provides a shorter, more convenient
/// way to create rests in musical compositions.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // These are equivalent
/// let rest1 = REST;
/// let rest2 = NoteKind::Rest;
///
/// // Use in note creation
/// let quarter_rest = quarter(REST);
/// ```
pub const REST: NoteKind = NoteKind::Rest;
