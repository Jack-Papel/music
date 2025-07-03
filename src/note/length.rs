use crate::{
    note::{chord::Chord, Timbre},
    Line, Note, NoteKind, NotePitch, Piece,
};

/// Represents the duration of a musical note in abstract time units.
///
/// The values for the unit system are based on common musical notation:
/// - `1` for sixteenth notes
/// - `2` for eighth notes  
/// - `4` for quarter notes
/// - `8` for half notes
/// - `16` for whole notes
///
/// If you want thirty-second notes, or further subdivisions this is not currently supported,
/// but you can increase the playback BPM to achieve a similar effect.
///
/// # Examples
/// Manually using `NoteLength`:
/// ```
/// use symphoxy::prelude::*;
///
/// // Create different note lengths
/// let quarter_len = NoteLength::new(4);
/// let half_len = NoteLength::new(8);
///
/// // Use with note creation functions
/// let note = Note(quarter_len, NoteKind::Rest);
///
/// // Convert from u16
/// let length: NoteLength = 4.into();
/// assert_eq!(length.duration(), 4);
/// ```
/// A more typical usage is through the `LengthFluid` trait:
/// ```
/// use symphoxy::prelude::*; // Imports LengthFluid trait and note length functions
///
/// // Create notes with specific lengths using fluent API
/// let note = quarter(A4);  // NotePitch -> Note
/// let rest_note = eighth(REST); // NoteKind -> Note
///
/// // Create a chord piece with half note length
/// let chord_piece = half(Chord::new([C4, A4])); // Chord -> Piece
///
/// // You can also use the `dotted` and `tie` functions for more complex durations
/// // `into()` is needed because these compound functions expect a `NoteKind`
/// let dotted_note = dotted(quarter)(C4); // Dotted quarter note
/// let tied_note = tie(quarter, eighth)(A4); // Tied quarter and eighth note
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NoteLength(pub u16);

/// A trait for types that can have their note length/duration modified.
///
/// This trait enables a fluent API for setting note durations using functions
/// like `quarter()`, `half()`, `whole()`, etc.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // These types implement LengthFluid:
/// let note = quarter(NotePitch::new(440.0));  // NotePitch -> Note
/// let rest_note = eighth(REST);               // NoteKind -> Note  
/// let chord_piece = half(Chord::new([C4, A4]));   // Chord -> Piece
/// ```
pub trait LengthFluid {
    /// The type returned after applying the length transformation
    type Output: HasNoteLength;

    /// Applies the specified note length to this musical element.
    ///
    /// This method transforms the element by setting its duration,
    /// potentially changing its type (e.g., `NotePitch` -> `Note`).
    fn with_length(self, length: NoteLength) -> Self::Output;
}

/// A trait for types that can provide their note length.
///
/// This trait is mainly used for note length functions such as `quarter()`, `eighth()`, etc.
pub trait HasNoteLength {
    /// Returns the length of this musical element as a `NoteLength`.
    fn length(&self) -> NoteLength;
}

macro_rules! note_length_fn {
    ($($name:ident, $value:expr, $doc:expr);*) => {
        $(
            #[doc = $doc]
            pub fn $name<N: LengthFluid>(kind: N) -> N::Output {
                kind.with_length(NoteLength($value))
            }
        )*
    }
}

impl LengthFluid for NoteKind {
    type Output = Note;

    fn with_length(self, length: NoteLength) -> Self::Output {
        Note(length, self)
    }
}

impl HasNoteLength for Note {
    fn length(&self) -> NoteLength {
        self.0
    }
}

impl LengthFluid for Note {
    type Output = Note;

    fn with_length(self, length: NoteLength) -> Self::Output {
        Note(length, self.1)
    }
}

impl LengthFluid for Chord {
    type Output = Piece;

    fn with_length(self, length: NoteLength) -> Self::Output {
        Piece(
            self.0
                .into_iter()
                .map(|note| note.with_length(length))
                .map(|note| Line {
                    notes: vec![note],
                    pickup: vec![],
                    hold_pickup: false,
                })
                .collect(),
        )
    }
}

impl HasNoteLength for Piece {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "This is intended only to be used by note length functions, which will only ever produce u16-sized things"
    )]
    fn length(&self) -> NoteLength {
        NoteLength(self.length() as u16)
    }
}

impl LengthFluid for NotePitch {
    type Output = Note;

    fn with_length(self, length: NoteLength) -> Self::Output {
        Note(
            length,
            NoteKind::Pitched {
                pitch: self,
                timbre: Timbre::Sine,
                volume: 1.0,
            },
        )
    }
}

note_length_fn!(
    sixteenth, 1, "Creates a sixteenth note (1 time unit) from the given musical element.";
    eighth, 2, "Creates an eighth note (2 time units) from the given musical element.";
    quarter, 4, "Creates a quarter note (4 time units) from the given musical element.";
    half, 8, "Creates a half note (8 time units) from the given musical element.";
    whole, 16, "Creates a whole note (16 time units) from the given musical element.";
    double_whole, 32, "Creates a double whole note (32 time units) from the given musical element."
);

/// Creates a dotted note with 1.5x the duration of the base note.
///
/// In music notation, a dot after a note increases its duration by half.
/// For example, a dotted quarter note lasts as long as a quarter note plus an eighth note.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Create dotted notes
/// let dotted_quarter = dotted(quarter)(C4);
/// let dotted_half = dotted(half)(REST);
///
/// // Dotted quarter = 4 + 2 = 6 time units
/// // Dotted half = 8 + 4 = 12 time units
/// ```
#[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
pub fn dotted<T: LengthFluid + Clone>(len_fn: impl Fn(T) -> T::Output) -> impl Fn(T) -> T::Output {
    Box::new(move |kind: T| {
        let out_length = len_fn(kind.clone()).length();
        kind.with_length(NoteLength(out_length.0 + out_length.0 / 2))
    })
}

/// Creates a tied note by combining the durations of two note lengths.
///
/// In music notation, a tie connects two notes of the same pitch,
/// effectively creating one longer note with their combined duration.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Tie a quarter note and eighth note together
/// let tied_note = tie(quarter, eighth)(C4);
/// // Duration = 4 + 2 = 6 time units
///
/// // Tie two half notes for a whole note (or you could just use the "whole" function)
/// let whole_via_tie = tie(half, half)(A4);
/// // Duration = 8 + 8 = 16 time units
/// ```
#[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
pub fn tie<T: LengthFluid + Clone>(
    len_fn1: impl Fn(T) -> T::Output,
    len_fn2: impl Fn(T) -> T::Output,
) -> impl Fn(T) -> T::Output {
    Box::new(move |kind: T| {
        let len1 = len_fn1(kind.clone());
        let len2 = len_fn2(kind.clone());
        kind.with_length(NoteLength(len1.length().0 + len2.length().0))
    })
}
