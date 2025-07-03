use crate::{Line, Note, NoteKind, Piece};

/// Defines the sound characteristics (timbre) of a musical note.
///
/// Timbre is what makes a piano sound different from a guitar playing the same note.
/// This enum provides built-in timbres as well as support for custom audio sources.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Use built-in timbres
/// let piano_note = piano(quarter(C4));
/// let guitar_note = electric_guitar(half(A4));
///
/// // Create notes with specific timbres
/// let sine_note = Note(4.into(), NoteKind::Pitched {
///     pitch: NotePitch::new(440.0),
///     timbre: Timbre::Sine,
///     volume: 1.0,
/// });
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Timbre {
    /// Pure sine wave - clean, simple tone with no harmonics
    #[default]
    Sine,

    /// Bass guitar sound - deep, rich low-frequency tones
    Bass,

    /// Piano sound - complex harmonic structure with natural decay
    Piano,

    /// Electric guitar sound - bright, sustained tones with distortion
    ElectricGuitar,

    /// Built-in drum kit sounds.
    ///
    /// The drum kit uses specific pitches to trigger different drum sounds:
    /// - **Kick drum**: C5 (523.25 Hz)
    /// - **Snare drum**: C4 (261.63 Hz)  
    /// - **Hi-hat**: C3 (130.81 Hz)
    /// - **Crash cymbal**: C6 (1046.5 Hz)
    ///
    /// # Example
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// // Create drum sounds using the mapped pitches
    /// let kick = drums(quarter(C4.octave(1)));  // C5
    /// let snare = drums(quarter(C4)); // C4
    /// let hihat = drums(quarter(C4.octave(-1))); // C3
    /// ```
    Drums,

    /// Custom unpitched audio source from a file.
    ///
    /// The audio file will be played back at its original pitch and duration,
    /// regardless of the note's pitch parameter. Useful for percussion,
    /// sound effects, or pre-recorded audio snippets.
    ///
    /// # Example
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let custom = Timbre::CustomSourceUnpitched("path/to/crash.mp3");
    /// let crash_note = Note(4.into(), NoteKind::Pitched {
    ///     pitch: A4, // Pitch ignored for unpitched sources
    ///     timbre: custom,
    ///     volume: 1.0,
    /// });
    /// ```
    CustomSourceUnpitched(&'static str),

    /// Custom pitched audio source from a file.
    ///
    /// The audio file is assumed to be recorded at C4 (261.63 Hz) pitch.
    /// When played, the audio will be pitch-shifted to match the note's
    /// frequency, allowing melodic use of custom samples.
    ///
    /// # Example
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let custom = Timbre::CustomSourcePitched("path/to/violin_c4.wav");
    /// let violin_a4 = Note(4.into(), NoteKind::Pitched {
    ///     pitch: NotePitch::new(440.0), // Will pitch-shift from C4 to A4
    ///     timbre: custom,
    ///     volume: 1.0,
    /// });
    /// ```
    CustomSourcePitched(&'static str),
}

/// A trait for types that can have their timbre (sound characteristics) modified.
///
/// This trait enables a fluent API for setting timbres using functions like
/// `piano()`, `bass()`, `electric_guitar()`, etc.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
///
/// // Apply timbres to different types
/// let piano_note = piano(quarter(C4)); // Note gets piano timbre
/// let bass_line = bass(Line::default()); // All notes in line get bass timbre  
/// let guitar_piece = electric_guitar(Piece::default()); // All notes in piece get guitar timbre
/// ```
// Couldn't think of a better name
pub trait TimbreFluid {
    /// Applies the specified timbre to this musical element.
    ///
    /// For single notes, this changes the timbre. For collections like
    /// lines and pieces, this applies the timbre to all contained notes.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    ///
    /// let note = quarter(C4).with_timbre(Timbre::Piano);
    /// let line = piano(quarter(C4) + quarter(A4));
    /// let electric_line = line.with_timbre(Timbre::ElectricGuitar);
    /// ```
    fn with_timbre(self, timbre: Timbre) -> Self;
}

impl TimbreFluid for NoteKind {
    fn with_timbre(self, timbre: Timbre) -> Self {
        match self {
            NoteKind::Pitched { pitch, volume, .. } => NoteKind::Pitched { pitch, timbre, volume },
            NoteKind::Rest => NoteKind::Rest,
        }
    }
}

impl TimbreFluid for Note {
    fn with_timbre(self, timbre: Timbre) -> Self {
        Note(self.0, self.1.with_timbre(timbre))
    }
}

impl TimbreFluid for Line {
    fn with_timbre(self, timbre: Timbre) -> Self {
        Line {
            notes: self.notes.into_iter().map(|n| n.with_timbre(timbre)).collect(),
            pickup: self.pickup.into_iter().map(|n| n.with_timbre(timbre)).collect(),
            hold_pickup: self.hold_pickup,
        }
    }
}

impl TimbreFluid for Piece {
    fn with_timbre(self, timbre: Timbre) -> Self {
        Piece(self.0.into_iter().map(|line| line.with_timbre(timbre)).collect())
    }
}

macro_rules! builtin_timbre_fns {
    ($($name:ident, $kind:ident, $doc:expr);*) => {
        $(
            #[doc = $doc]
            pub fn $name<T: TimbreFluid>(timbre_haver: T) -> T {
                timbre_haver.with_timbre(Timbre::$kind)
            }
        )*
    }
}

builtin_timbre_fns!(
    sine, Sine, "Applies a pure sine wave timbre - clean, simple tone with no harmonics.";
    bass, Bass, "Applies a bass guitar timbre - deep, rich low-frequency tones.";
    piano, Piano, "Applies a piano timbre - complex harmonic structure with natural decay.";
    electric_guitar, ElectricGuitar, "Applies an electric guitar timbre - bright, sustained tones with distortion.";
    drums, Drums, "Applies a drum kit timbre - use specific pitches to trigger different drum sounds."
);
