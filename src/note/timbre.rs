use crate::{Line, Note, NoteKind, Piece};

#[derive(Clone, Copy, Debug)]
pub enum Timbre {
    Sine,
    Bass,
    Piano,
    ElectricGuitar,
    /// Built-in drums.
    /// A brief rundown of the kit is as follows:
    /// - Kick: C5
    /// - Snare: C4
    /// - Hi-hat: C3
    /// - Crash: C6
    Drums,
    /// Custom timbre, e.g. a custom audio file
    /// The string is the path to the audio file
    CustomSourceUnpitched(&'static str),
    /// Custom timbre, e.g. a custom audio file
    /// The string is the path to the audio file
    /// This is for pitched sources, e.g. a sample that can be played at different pitches
    /// The pitch is assumed to be in C4, so the frequency will be adjusted accordingly
    CustomSourcePitched(&'static str),
}

// Couldn't think of a better name
pub trait TimbreFluid {
    fn with_timbre(self, timbre: Timbre) -> Self;
}

impl TimbreFluid for NoteKind {
    fn with_timbre(self, timbre: Timbre) -> Self {
        match self {
            NoteKind::Pitched { pitch, volume, .. } => NoteKind::Pitched {
                pitch,
                timbre,
                volume
            },
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
            notes: self.notes
                .into_iter()
                .map(|n| n.with_timbre(timbre))
                .collect(),
            pickup: self.pickup
                .into_iter()
                .map(|n| n.with_timbre(timbre))
                .collect(),
            hold_pickup: self.hold_pickup
        }
    }
}

impl TimbreFluid for Piece {
    fn with_timbre(self, timbre: Timbre) -> Self {
        Piece(
            self.0.into_iter()
                .map(|line| line.with_timbre(timbre))
                .collect()
        )
    }
}

macro_rules! builtin_timbre_fns {
    ($($name:ident, $kind:ident);*) => {
        $(
            pub fn $name<T: TimbreFluid>(timbre_haver: T) -> T {
                timbre_haver.with_timbre(Timbre::$kind)
            }
        )*
    }
}

builtin_timbre_fns!(
    sine, Sine;
    bass, Bass;
    piano, Piano;
    electric_guitar, ElectricGuitar;
    drums, Drums
);