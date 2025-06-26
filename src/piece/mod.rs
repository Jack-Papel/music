use std::{fmt::Write, ops::{Add, Mul}};

use iter_tools::{EitherOrBoth, Itertools};
use line::Line;

use crate::{note::{NoteKind, NotePitch, Timbre}, scales::tet12::{self, A4, C4}, Note, Tet12};

pub mod line;

#[derive(Clone, Default)]
pub struct Piece(pub Vec<Line>);

impl Piece {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn volume(&self, volume: f32) -> Self {
        Piece(self.0.iter().map(|line| line.volume(volume)).collect())
    }
}

impl From<Line> for Piece {
    fn from(value: Line) -> Self {
        Piece(vec![value])
    }
}

impl From<Note> for Piece {
    fn from(value: Note) -> Self {
        Piece(vec![Line::new_unchecked(vec![value])])
    }
}

impl Piece {
    pub(crate) fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item=Note> {
        self.0.clone()
            .into_iter()
            .flat_map(move |l| l.get_notes_at_instant(instant).collect::<Vec<_>>())
    }

    /// As opposed to `get_notes_at_instant`, this gets any note which would
    /// be playing during a given instant, rather than the notes which start at a given instant.
    #[expect(clippy::arithmetic_side_effects, reason = "Manual bounds checking, almost always safe")]
    fn get_notes_during_instant(&self, instant: usize) -> impl Iterator<Item=Note> {
        self.0.clone()
            .into_iter()
            .filter_map(move |l| {
                // get note at time
                let mut time_acc = 0;
                for note in l.notes.clone() {
                    if time_acc <= instant && instant < time_acc + note.0.0 as usize {
                        return Some(note);
                    }
                    time_acc += note.0.0 as usize;
                }

                None
            })
    }

    pub fn length(&self) -> usize {
        self.0.iter()
            .map(|line| line.length())
            .max()
            .unwrap_or_default()
    }
}

impl Mul<Piece> for Piece {
    type Output = Piece;

    fn mul(self, rhs: Piece) -> Self::Output {
        Piece([self.0, rhs.0].concat())
    }
}

impl Mul<usize> for Piece {
    type Output = Piece;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    fn mul(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return Piece::new();
        }
        
        let mut acc = self.clone();
        for _ in 1..rhs {
            acc = acc + self.clone()
        }
        acc
    }
}

impl Add<Piece> for Piece {
    type Output = Piece;

    #[expect(clippy::arithmetic_side_effects, reason = "Arithmetic implementation")]
    #[expect(clippy::cast_possible_truncation, reason = "I don't want to deal with this right now")]
    fn add(self, rhs: Piece) -> Self::Output {
        let self_length = self.length() as u16;
        let rhs_length = rhs.length() as u16;
        Piece(
            self.0.into_iter().zip_longest(rhs.0.iter())
                .map(|either_or_both| {
                    match either_or_both {
                        EitherOrBoth::Both(first, second) => first.clone() + second.clone(),
                        EitherOrBoth::Left(first) => first.clone().extend(rhs_length),
                        EitherOrBoth::Right(second) => Line::new().extend(self_length) + second.clone()
                    }
                    
                })
                .collect()
        )
    }
}

impl Mul<Line> for Piece {
    type Output = Piece;

    #[expect(clippy::cast_possible_truncation, reason = "I don't want to deal with this right now")]
    fn mul(self, rhs: Line) -> Self::Output {
        let self_len = self.length();
        let rhs_len = rhs.length();
        let new_len = usize::max(self_len, rhs_len);
        
        // Extend pieces to same length for layering
        let extended_self: Vec<_> = self.0.into_iter()
            .map(|line| {
                let padding = new_len.saturating_sub(self_len) as u16;
                line.extend(padding)
            })
            .collect();
        
        let padding = new_len.saturating_sub(rhs_len) as u16;
        let extended_rhs = vec![rhs.extend(padding)];
        
        Piece([extended_self, extended_rhs].concat())
    }
}

impl Mul<Note> for Piece {
    type Output = Piece;

    fn mul(self, rhs: Note) -> Self::Output {
        Piece([self.0, vec![rhs.into()]].concat())
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let black_keys = [false, true, false, true, false, false, true, false, true, false, true, false];

        for bar_group in 0..self.length().div_ceil(64) {
            let (highest_semitone, lowest_semitone) = {
                let (mut highest, mut lowest) = (i16::MIN, i16::MAX);
                #[expect(clippy::arithmetic_side_effects, reason = "Guaranteed to be safe, manual bounds checking")]
                for time in (bar_group * 64)..(bar_group * 64 + 64) {
                    for note in self.get_notes_during_instant(time) {
                        if let NoteKind::Pitched { pitch: NotePitch(frequency), .. } = note.1 {
                            let semitone_diff_from_c4 = 12.0 * f32::log2(frequency / C4.0);

                            #[expect(clippy::cast_possible_truncation, reason = "Intentional precision loss")]
                            if highest < semitone_diff_from_c4 as i16 {
                                highest = semitone_diff_from_c4 as i16;
                            } else if lowest > semitone_diff_from_c4 as i16 {
                                lowest = semitone_diff_from_c4 as i16;
                            }
                        }
                    }
                }
                (highest, lowest)
            };
            f.write_str(&"═".repeat(74))?;
            f.write_str("╗\n")?;
            #[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
            for semitone in (lowest_semitone - 2..=highest_semitone + 2).rev() {
                let pitch = C4.semitone(semitone);
                let mut line_str = String::new();

                if [4, -1, -5, -10, -15, -20].contains(&semitone) {
                    f.write_char('!')?;
                } else {
                    f.write_char(' ')?;
                }

                for bar_group_time in 0..64 {
                    let time = 64 * bar_group + bar_group_time;
                    let black_key = black_keys[(semitone.rem_euclid(12)) as usize];

                    // Add barline
                    if bar_group_time % 16 == 0 {
                        if bar_group_time == 0 {
                            line_str.push_str(&format!("{: <3}", tet12::get_note_name(pitch, A4)));
                            if black_key {
                                line_str.push_str("║ ║");
                            } else {
                                line_str.push_str("║█║");
                            }
                        } else {
                            line_str.push('|');
                        }
                    }
    
                    let blank_space = if black_key {
                        ' '
                    } else {
                        '░'
                    };

                    let note_matches_line = |note: &Note| {
                        match note.1 {
                            NoteKind::Rest => false,
                            NoteKind::Pitched{ pitch: note_pitch, timbre, .. } => 
                                !matches!(timbre, Timbre::Drums) && 
                                (note_pitch.0 / pitch.0 - 1.0).abs() < (2.0f32.powf(1.0/24.0) - 1.0)
                        }
                    };
    
                    // Find notes at this time on this line
                    if let Some(_note) = self.get_notes_at_instant(time).find(note_matches_line) {
                        line_str.push('■');
                    } else if let Some(_note) = self.get_notes_during_instant(time).find(note_matches_line) {
                        line_str.push('≡');
                    } else {
                        line_str.push(blank_space);
                    }
                }
                line_str.push_str("║\n");
                f.write_str(&line_str)?;
            }
            f.write_str(&("═".repeat(74) + "╣" + "\n"))?;
            for kind in ["crash", "hi-hat", "snare", "kick"] {
                let mut line_str = String::new();

                for bar_group_time in 0..64 {
                    #[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
                    let time = 64 * bar_group + bar_group_time;

                    // Add barline
                    if bar_group_time % 16 == 0 {
                        if bar_group_time == 0 {
                            line_str.push_str(&format!("{: <6}", kind));
                            line_str.push('║');
                        } else {
                            line_str.push('|');
                        }
                    }

                    let note_matches_line = |note: &Note| {
                        match note.1 {
                            NoteKind::Rest => false,
                            NoteKind::Pitched{ pitch, timbre, .. } => {
                                matches!(timbre, crate::note::Timbre::Drums) &&
                                match kind {
                                    "crash" => pitch.0 > C4.octave(1).semitone(6).0,
                                    "hi-hat" => C4.octave(1).semitone(6).0 > pitch.0 && pitch.0 > C4.semitone(6).0,
                                    "snare" => C4.semitone(-6).0 < pitch.0 && pitch.0 < C4.semitone(6).0,
                                    "kick" => pitch.0 < C4.semitone(-6).0,
                                    _ => false
                                }
                            }
                        }
                    };
    
                    // Find notes at this time on this line
                    if let Some(_note) = self.get_notes_at_instant(time).find(note_matches_line) {
                        line_str.push('■');
                    } else if let Some(_note) = self.get_notes_during_instant(time).find(note_matches_line) {
                        line_str.push('≡');
                    } else {
                        line_str.push(' ');
                    }
                }
                line_str.push_str("║\n");
                f.write_str(&line_str)?;
            }
            f.write_str(&"═".repeat(74))?;
            f.write_str("╝\n\n\n")?;
        }

        Ok(())
    }
}
