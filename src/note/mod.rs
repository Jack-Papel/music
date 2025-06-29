mod timbre;
mod length;
pub mod chord;

pub use timbre::*;
pub use length::*;

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
        write!(f, "{}", crate::scales::tet12::get_note_name_with_octave(*self, A4))
    }
}

pub const REST: NoteKind = NoteKind::Rest;
