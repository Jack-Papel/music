use std::ops::Add;

use crate::{Line, Note, NoteKind, NotePitch, Piece, Scale, Tet12, C4};

#[derive(Clone)]
pub struct Chord(pub Vec<NotePitch>);

impl Chord {
    pub fn new(pitches: impl IntoIterator<Item=NotePitch>) -> Self {
        Chord(pitches.into_iter().collect())
    }

    pub fn from_degrees(scale: &impl Scale, degrees: &[isize]) -> Self {
        let pitches = degrees.iter().map(|&degree| scale.get_degree(degree)).collect();
        Chord(pitches)
    }

    pub fn strike(&self, striker: fn(NotePitch) -> Line) -> Piece {
        Piece(self.0.iter().map(|&pitch| striker(pitch)).collect())
    }

    /// Transposes the chord to a new target pitch.
    /// If the chord is empty, it returns a clone of itself.
    /// The transposition is done by scaling the pitches so that the lowest pitch matches the target pitch.
    pub fn transpose_to(&self, target: NotePitch) -> Self {
        if self.0.is_empty() {
            return self.clone();
        }
        let offset = target.0 / self.0.iter().map(|p| p.0).reduce(f32::min).unwrap();
        Chord(self.0.iter().map(|&pitch| NotePitch(pitch.0 * offset)).collect())
    }

    /// Creates a chord from a shape defined by semitone offsets from C4.
    /// C4 is already included, so only the offsets are needed.
    pub fn shape_from_semitone_offsets<const I: usize>(semitones: [u8; I]) -> Self {
        let mut out = vec![C4];
        let pitches = semitones.map(|s| C4.semitone(s as i16));
        for pitch in pitches {
            out.push(pitch);
        }
        Chord(out)
    }
}

pub trait ChordFluid {
    type Output;

    /// Creates a chord with the given shape, transposed so that the lowest pitch of the chord meets the relevant pitch.
    fn with_chord_shape(self, chord_shape: &Chord) -> Self::Output;
}

impl ChordFluid for NotePitch {
    type Output = Chord;

    fn with_chord_shape(self, chord_shape: &Chord) -> Self::Output {
        chord_shape.transpose_to(self)
    }
}

impl ChordFluid for Note {
    type Output = Piece;

    fn with_chord_shape(self, chord_shape: &Chord) -> Self::Output {
        match self.1 {
            NoteKind::Rest => Piece(vec![Line {
                notes: vec![self],
                pickup: vec![],
                hold_pickup: false,
            }]),
            NoteKind::Pitched { pitch, timbre, volume } => {
                let chord = pitch.with_chord_shape(chord_shape);

                Piece(
                    chord.0.into_iter().map(|note_pitch| {
                        Line {
                            notes: vec![Note(self.0, NoteKind::Pitched { pitch: note_pitch, timbre, volume })],
                            pickup: vec![],
                            hold_pickup: false,
                        }
                    }).collect()
                )
            }
        }
    }
}

impl ChordFluid for Line {
    type Output = Piece;

    fn with_chord_shape(self, chord_shape: &Chord) -> Self::Output {
        self.notes.into_iter().map(|note| note.with_chord_shape(chord_shape)).reduce(Add::add)
            .unwrap_or_else(|| Piece(vec![]))
    }
}