use std::{fmt::Debug, ops::{Add, Mul}};

use crate::{A4, Line};

/// Represents a musical note with duration, pitch/rest, and timbre
/// 
/// # Examples
/// ```
/// use symphoxy::prelude::*;
/// 
/// // Create a quarter note C4 with piano timbre
/// let note = piano(quarter(NotePitch(261.626)));
/// ```
#[derive(Clone, Copy)]
pub struct Note(pub NoteLength, pub NoteKind);

impl Note {
    pub fn volume(&self, volume: f32) -> Note {
        let new_note_kind = match self.1 {
            NoteKind::Pitched { pitch, timbre, .. } => NoteKind::Pitched { pitch, timbre, volume },
            NoteKind::Rest => NoteKind::Rest,
        };
        
        Note(
            self.0,
            new_note_kind
        )
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Note").field(&self.0).field(&self.1).finish()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Timbre {
    Sine,
    Bass,
    Piano,
    ElectricGuitar,
    Drums
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
        Line::new_unchecked(Vec::from_iter(
            std::iter::repeat(self)
                .take(rhs)
        ))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum NoteKind {
    Rest,
    /// Frequency
    Pitched{
        pitch: NotePitch,
        timbre: Timbre,
        volume: f32
    }
}

impl From<NotePitch> for NoteKind {
    fn from(value: NotePitch) -> Self {
        NoteKind::Pitched{
            pitch: value,
            timbre: Timbre::Sine,
            volume: 1.0
        }
    }
}

#[derive(Clone, Copy)]
pub struct NotePitch(pub f32);

impl Debug for NotePitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::scales::tet12::get_note_name(*self, A4))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoteLength(pub u16);

pub const REST: NoteKind = NoteKind::Rest;


pub mod note_length_fns {
    use super::*;

    macro_rules! note_length_fn {
        ($($name:ident, $value:expr);*) => {
            $(
                pub fn $name(kind: impl Into<NoteKind>) -> Note {
                    Note(NoteLength($value), kind.into())
                }
            )*
        }
    }

    note_length_fn!(
        sixteenth, 1;
        eighth, 2;
        quarter, 4;
        half, 8
    );

    #[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
    pub fn dotted(len_fn: impl Fn(NoteKind) -> Note) -> impl Fn(NoteKind) -> Note {
        Box::new(move |kind| {
            let len = len_fn(kind).0.0;
            Note(NoteLength(len + len / 2), kind)
        })
    }
    
    #[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
    pub fn tie(len_fn1: impl Fn(NoteKind) -> Note, len_fn2: impl Fn(NoteKind) -> Note) -> impl Fn(NoteKind) -> Note {
        Box::new(move |kind| Note(NoteLength(len_fn1(kind).0.0 + len_fn2(kind).0.0), kind))
    }
}

macro_rules! timbre_fns {
    ($($name:ident, $kind:ident);*) => {
        pub mod timbre_fns {
            use super::*;

            $(
                pub fn $name(line: impl Into<Line>) -> Line {
                    let line = line.into();

                    fn replace_timbre(note_kind: NoteKind, timbre: Timbre) -> NoteKind {
                        match note_kind {
                            NoteKind::Pitched { pitch, volume, .. } => NoteKind::Pitched {
                                pitch,
                                timbre,
                                volume
                            },
                            NoteKind::Rest => NoteKind::Rest,
                        }
                    }

                    Line {
                        notes: line.notes
                            .into_iter()
                            .map(|n| Note(n.0, replace_timbre(n.1, Timbre::$kind)))
                            .collect(),
                        pickup: line.pickup
                            .into_iter()
                            .map(|n| Note(n.0, replace_timbre(n.1, Timbre::$kind)))
                            .collect(),
                        hold_pickup: line.hold_pickup
                    }
                }
            )*
        }
    }
}

timbre_fns!(
    sine, Sine;
    bass, Bass;
    piano, Piano;
    electric_guitar, ElectricGuitar;
    drums, Drums
);