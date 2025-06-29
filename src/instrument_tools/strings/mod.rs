use crate::{note::chord::Chord, NotePitch, Tet12};


///
/// ## Example
/// ```
/// use symphoxy::instrument_tools::strings::{GuitarTuning, GuitarFrets};
/// use symphoxy::prelude::*;
/// use symphoxy::scales::tet12::{A4, get_note_name};
/// 
/// // GuitarTuning is an alias for StringTuning<6>
/// let tuning = GuitarTuning::DEFAULT_GUITAR_TUNING;
/// 
/// let g_major_frets = GuitarFrets::new_full([3, 0, 0, 0, 2, 3]);
/// 
/// let g_major_chord = tuning.get_chord(&g_major_frets);
/// 
/// // Check each note is a G, B, or D
/// assert!(g_major_chord.get_notes_at_instant(0).all(|pitch| match pitch.1 {
///   NoteKind::Pitched { pitch, .. } => {
///     get_note_name(pitch, A4) == "G" ||
///     get_note_name(pitch, A4) == "B" ||
///     get_note_name(pitch, A4) == "D"
///   },
///   NoteKind::Rest => false,
/// }));
/// ```
pub struct Frets<const N: usize>(pub [Option<i16>; N]);

pub type GuitarFrets = Frets<6>;

impl<const N: usize> Frets<N> {
    pub fn new_full(frets: [i16; N]) -> Self {
        Frets(frets.map(Some))
    }
}

impl<const N: usize> From<[i16; N]> for Frets<N> {
    fn from(frets: [i16; N]) -> Self {
        Frets(frets.map(Some))
    }
}

impl GuitarFrets {
    /// Creates a chord where the first four strings are specified and the last two are muted.
    pub fn new_four_string(frets: [i16; 4]) -> Self {
        let mut full_frets = [None; 6];
        full_frets[0..4].copy_from_slice(&frets.map(Some));
        Frets(full_frets)
    }
}

pub struct StringTuning<const N: usize>(pub [NotePitch; N]);

pub type GuitarTuning = StringTuning<6>;

impl<const N: usize> StringTuning<N> {
    pub fn new(tuning: [NotePitch; N]) -> Self {
        StringTuning(tuning)
    }

    /// # Safety
    /// This function is unsafe because it does not check if the string index is within bounds.
    /// If the index is out of bounds, it will panic.
    pub unsafe fn get_pitch_unchecked(&self, string: usize, fret: i16) -> NotePitch {
        self.0[string].semitone(fret)
    }

    pub fn get_pitches_at_frets(&self, frets: &Frets<N>) -> [Option<NotePitch>; N] {
        let mut pitches = [None; N];

        for (i, &fret) in frets.0.iter().enumerate() {
            if let Some(f) = fret {
                pitches[i] = Some(unsafe { self.get_pitch_unchecked(i, f) } );
            }
        }

        pitches
    }

    pub fn get_chord(&self, frets: &Frets<N>) -> Chord {
        let pitches = self.get_pitches_at_frets(frets);
        
        Chord::new(pitches.iter().filter_map(|pitch| *pitch))
    }
}

impl GuitarTuning {
    pub const GUITAR_HIGH_E: NotePitch = NotePitch(329.6);
    pub const GUITAR_B: NotePitch = NotePitch(246.9);
    pub const GUITAR_G: NotePitch = NotePitch(196.0);
    pub const GUITAR_D: NotePitch = NotePitch(146.8);
    pub const GUITAR_A: NotePitch = NotePitch(110.0);
    pub const GUITAR_LOW_E: NotePitch = NotePitch(82.41);

    pub const DEFAULT_GUITAR_TUNING: GuitarTuning = StringTuning ([
        Self::GUITAR_HIGH_E, 
        Self::GUITAR_B, 
        Self::GUITAR_G, 
        Self::GUITAR_D, 
        Self::GUITAR_A, 
        Self::GUITAR_LOW_E
    ]);
}