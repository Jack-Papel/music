use crate::{note::chord::Chord, NotePitch, Tet12};

/// Represents fret positions on a string instrument.
///
/// This type stores the fret positions for each string, where `None` indicates
/// a muted string and `Some(fret_number)` indicates which fret to press.
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
/// assert!(g_major_chord.0.into_iter().all(|pitch|
///     get_note_name(pitch, A4) == "G" ||
///     get_note_name(pitch, A4) == "B" ||
///     get_note_name(pitch, A4) == "D"
/// ));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Frets<const N: usize>(pub [Option<i16>; N]);

/// Type alias for guitar fret positions (6 strings). See [`Frets`] for details.
pub type GuitarFrets = Frets<6>;

impl<const N: usize> Frets<N> {
    /// Creates a new fret configuration where all strings are fretted.
    ///
    /// Takes an array of fret numbers and converts them to `Some(fret)` for each string.
    /// No strings will be muted with this constructor.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::instrument_tools::strings::*;
    ///
    /// let all_fretted = GuitarFrets::new_full([3, 2, 0, 0, 3, 3]); // All strings active
    /// ```
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

/// Represents the tuning of a string instrument.
///
/// This type stores the open string pitches for an N-string instrument.
/// Combined with fret positions, it can calculate the resulting pitches
/// and generate chords.
///
/// # Examples
/// ```
/// use symphoxy::prelude::*;
/// use symphoxy::instrument_tools::strings::*;
///
/// // Standard guitar tuning (low to high: E-A-D-G-B-E)
/// let guitar_tuning = GuitarTuning::DEFAULT_GUITAR_TUNING;
///
/// // Create a simple G major chord shape
/// let g_major_frets = GuitarFrets::new_full([3, 0, 0, 0, 2, 3]);
/// let chord = guitar_tuning.get_chord(&g_major_frets);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StringTuning<const N: usize>(pub [NotePitch; N]);

/// Type alias for guitar tuning (6 strings). See [`StringTuning`] for details.
pub type GuitarTuning = StringTuning<6>;

impl<const N: usize> StringTuning<N> {
    /// Creates a new string tuning from an array of pitches.
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    /// use symphoxy::instrument_tools::strings::*;
    ///
    /// let custom_tuning = StringTuning::new([
    ///     NotePitch::new(82.41),  // Low E
    ///     NotePitch::new(110.0),  // A
    ///     NotePitch::new(146.8),  // D
    ///     NotePitch::new(196.0),  // G
    /// ]);
    /// ```
    pub fn new(tuning: [NotePitch; N]) -> Self {
        StringTuning(tuning)
    }

    /// # Safety
    /// This function is unsafe because it does not check if the string index is within bounds.
    /// If the index is out of bounds, it will panic.
    pub unsafe fn get_pitch_unchecked(&self, string: usize, fret: i16) -> NotePitch {
        self.0[string].semitone(fret)
    }

    /// Gets the pitches produced by the given fret configuration.
    ///
    /// Returns an array where each element corresponds to a string:
    /// - `Some(pitch)` if the string is fretted
    /// - `None` if the string is muted
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    /// use symphoxy::instrument_tools::strings::*;
    ///
    /// let tuning = GuitarTuning::DEFAULT_GUITAR_TUNING;
    /// let frets = GuitarFrets::new_full([0, 0, 0, 0, 0, 0]); // All open strings
    /// let pitches = tuning.get_pitches_at_frets(&frets);
    /// // Returns the open string pitches
    /// ```
    pub fn get_pitches_at_frets(&self, frets: &Frets<N>) -> [Option<NotePitch>; N] {
        let mut pitches = [None; N];

        for (i, &fret) in frets.0.iter().enumerate() {
            if let Some(f) = fret {
                pitches[i] = Some(unsafe { self.get_pitch_unchecked(i, f) });
            }
        }

        pitches
    }

    /// Creates a chord from the given fret configuration.
    ///
    /// This combines the tuning with the fret positions to generate
    /// a chord containing all the sounding pitches (muted strings are excluded).
    ///
    /// # Examples
    /// ```
    /// use symphoxy::prelude::*;
    /// use symphoxy::instrument_tools::strings::*;
    ///
    /// let tuning = GuitarTuning::DEFAULT_GUITAR_TUNING;
    /// let g_major = GuitarFrets::new_full([3, 2, 0, 0, 3, 3]);
    /// let chord = tuning.get_chord(&g_major);
    /// // Creates a G major chord
    /// ```
    pub fn get_chord(&self, frets: &Frets<N>) -> Chord {
        let pitches = self.get_pitches_at_frets(frets);

        Chord::new(pitches.iter().filter_map(|pitch| *pitch))
    }
}

impl GuitarTuning {
    /// High E string pitch (1st string) - approximately 329.6 Hz
    pub const GUITAR_HIGH_E: NotePitch = NotePitch(329.6);
    /// B string pitch (2nd string) - approximately 246.9 Hz  
    pub const GUITAR_B: NotePitch = NotePitch(246.9);
    /// G string pitch (3rd string) - approximately 196.0 Hz
    pub const GUITAR_G: NotePitch = NotePitch(196.0);
    /// D string pitch (4th string) - approximately 146.8 Hz
    pub const GUITAR_D: NotePitch = NotePitch(146.8);
    /// A string pitch (5th string) - approximately 110.0 Hz
    pub const GUITAR_A: NotePitch = NotePitch(110.0);
    /// Low E string pitch (6th string) - approximately 82.41 Hz
    pub const GUITAR_LOW_E: NotePitch = NotePitch(82.41);

    /// Standard guitar tuning (E-A-D-G-B-E from low to high)
    ///
    /// This represents the most common guitar tuning used in Western music.
    /// The strings are ordered from highest pitch (1st string) to lowest pitch (6th string)
    /// to match typical guitar tablature conventions.
    pub const DEFAULT_GUITAR_TUNING: GuitarTuning = StringTuning([
        Self::GUITAR_HIGH_E,
        Self::GUITAR_B,
        Self::GUITAR_G,
        Self::GUITAR_D,
        Self::GUITAR_A,
        Self::GUITAR_LOW_E,
    ]);
}
