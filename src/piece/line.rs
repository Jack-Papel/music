use std::ops::{Add, Mul, Neg, Not};

use crate::{note::{NoteKind, NoteLength}, Note, Playable};

use super::Piece;


#[derive(Clone, Debug, Default)]
pub struct Line{
    pub notes: Vec<Note>,
    pub pickup: Vec<Note>,
    pub hold_pickup: bool,
}

impl Line {
    pub fn new() -> Line {
        Line::default()
    }
    pub fn new_unchecked(notes: Vec<Note>) -> Line {
        Line {
            notes,
            pickup: vec![],
            hold_pickup: false
        }
    }
    pub fn extend(&self, extend_by: u16) -> Self {
        if extend_by == 0 {
            return self.clone()
        }
        self.clone() + Note(
            NoteLength(extend_by),
            NoteKind::Rest,
            crate::note::Timbre::Sine
        )
    }
}

impl Neg for Line {
    type Output = Line;

    fn neg(self) -> Self::Output {
        Self {
            notes: vec![],
            pickup: self.notes,
            hold_pickup: self.hold_pickup
        }
    }
}

impl Not for Line {
    type Output = Line;

    fn not(self) -> Self::Output {
        Self {
            hold_pickup: true,
            ..self
        }
    }
}

impl From<Note> for Line {
    fn from(value: Note) -> Self {
        Line::new_unchecked(vec![value])
    }
}

impl Add<Piece> for Line {
    type Output = Piece;

    /// This implementation puts this line as the first line of the piece
    fn add(self, rhs: Piece) -> Self::Output {
        if !rhs.0.is_empty() {
            let mut piece = rhs.clone();
            let self_len = self.length();

            piece.0[0] = self + piece.0[0].clone();
            for line_no in 1..piece.0.len() {
                piece.0[line_no] = Line::new().extend(self_len as u16) + piece.0[line_no].clone()
            }

            piece
        } else {
            self.into()
        }
    }
}

impl Add<Note> for Line {
    type Output = Line;

    fn add(self, rhs: Note) -> Self::Output {
        self + Line::new_unchecked(vec![rhs])
    }
}

impl Add<Line> for Line {
    type Output = Line;

    fn add(self, rhs: Line) -> Self::Output {
        let mut notes = self.notes.clone();

        let mut pickup_line = Line::new_unchecked(rhs.pickup);
        let pickup_length = pickup_line.length();

        let mut time_removed = 0;
        let mut notes_to_remove = 0;
        let mut note_to_add = None;
        for note in notes.iter().rev() {
            if pickup_length <= time_removed {
                break;
            }

            if pickup_length >= time_removed + note.0.0 as usize{
                time_removed += note.0.0 as usize;
                notes_to_remove += 1;
            } else {
                // Need to remove part of a note
                notes_to_remove += 1;
                note_to_add = Some(Note(
                    NoteLength(note.0.0 - (pickup_length - time_removed) as u16),
                    note.1,
                    note.2
                ));
                break;
            }
        }

        for _ in 0..notes_to_remove {
            notes.pop();
        }

        if let Some(note) = note_to_add {
            notes.push(note);
        }

        notes.append(&mut pickup_line.notes);

        let mut rhs_notes = rhs.notes;

        if rhs.hold_pickup {
            if let Some(last_note) = notes.iter().last() {
                let last_index = notes.len() - 1;

                notes[last_index] = Note(
                    NoteLength(last_note.0.0 + rhs_notes[0].0.0),
                    last_note.1,
                    last_note.2
                );

                rhs_notes.remove(0);
            }
        }

        Line {
            notes: [notes, rhs_notes].concat(),
            pickup: self.pickup,
            hold_pickup: self.hold_pickup
        }
    }
}

impl Mul<usize> for Line {
    type Output = Line;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut current_line = self.clone();

        for _ in 0..(rhs - 1) {
            current_line = current_line + self.clone();
        }

        current_line
    }
}

impl Mul<Line> for Line {
    type Output = Piece;

    fn mul(self, rhs: Line) -> Self::Output {
        Piece(vec![self, rhs])
    }
}

impl Playable for Line {
    /// Returns the length of this line without regard for the pickup
    fn length(&self) -> usize {
        let mut acc = 0;
        for note in self.notes.clone() {
            acc += note.0.0 as usize;
        }
        acc
    }

    fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item=Note> {
        let mut time_acc = 0;
        for note in self.notes.clone() {
            if time_acc == instant {
                return Some(note).into_iter();
            }
            time_acc += note.0.0 as usize
        }

        None.into_iter()
    }
}