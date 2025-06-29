use crate::{note::{chord::Chord, Timbre}, Line, Note, NoteKind, NotePitch, Piece};


#[derive(Clone, Copy, Debug)]
pub struct NoteLength(pub u16);

pub trait LengthFluid {
    type Output;

    fn with_length(self, length: NoteLength) -> Self::Output;
}

macro_rules! note_length_fn {
    ($($name:ident, $value:expr);*) => {
        $(
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
            self.0.into_iter()
                .map(|note| note.with_length(length))
                .map(|note| Line {
                    notes: vec![note],
                    pickup: vec![],
                    hold_pickup: false,
                }).collect()
        )
    }
}

impl LengthFluid for NotePitch {
    type Output = Note;

    fn with_length(self, length: NoteLength) -> Self::Output {
        Note(length, NoteKind::Pitched { pitch: self, timbre: Timbre::Sine, volume: 1.0 })
    }
}

note_length_fn!(
    sixteenth, 1;
    eighth, 2;
    quarter, 4;
    half, 8;
    whole, 16;
    double_whole, 32
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