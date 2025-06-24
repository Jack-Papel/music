//! # Jack Papel's music-as-code software
//! 
//! ## Author
//! 
//! [Jack Papel](https://www.github.com/Jack-Papel)
//! 
//! ## About the library
//! 
//! I didn't want this library to be over-reliant on music notation, I wanted it to be more like 
//! a piano roll. Though since I have a large familiarity with music theory, most of the 
//! terminology is based on traditional music theory.
//! 
//! ## How it works:
//! 
//! Basically, when you concatenate notes (+), you get a "line" of notes.
//! When you stack lines or notes, (*), you get a "piece" which contains several lines played at 
//! once
//!
//! ## Some corners you'll run into
//! 
//! * I made it so you can add (+) notes to lines, and lines to pieces, and you can multiply most 
//!   things, but I probably forgot a few. Like, I think right now you can't add notes to pieces 
//!   or lines to notes without converting.
//! * Lines can have pickups. 
//!   * The pickup is played only if the line has been concatenated onto one before it. When this
//!     is done, the pickup overwrites whatever the previous line had
//!   * If you do -line then the line will turn into a pickup line.
//!   * If you do !line then that line's pickup will be held into its first note
//! * Pitches and NoteKinds are different
//!   * A NoteKind may be pitched, or it may be a rest.
//!   * Most functions accept impl Into<NoteKind>, which Pitch implements, however some don't.
//!     * Particulary dotted(eighth) returns a function which only accepts NoteKind, not 
//!       impl Into<NoteKind>

#![deny(clippy::arithmetic_side_effects)]
#![warn(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss, clippy::cast_sign_loss)]
#![deny(clippy::allow_attributes_without_reason)]
#![deny(clippy::allow_attributes)]


pub mod piece;
pub mod note;
pub mod scales;
mod interactive;
mod render_to_wav;

use std::sync::Arc;
use std::thread::JoinHandle;

pub use piece::Piece;
pub use piece::line::Line;
pub use note::Note;

pub use crate::interactive::InteractiveCli;

/// Creates a configuration for this music library
pub struct MusicPlayer<O: MusicOutput + Clone> {
    /// Tempo in beats per minute (default: 300 BPM which gives 200ms per beat)
    tempo_bpm: u32,
    output_config: O,
}

impl<O: MusicOutput + Clone> MusicPlayer<O> {
    /// Calculate milliseconds per beat based on BPM
    pub fn beat_duration_ms(&self) -> u64 {
        60_000u64.checked_div(self.tempo_bpm as u64).unwrap_or(u64::MAX)
    }
}

impl MusicPlayer<LiveOutputConfig> {
    pub fn new_live(tempo_bpm: u32, output_handle: Arc<rodio::OutputStreamHandle>) -> Self {
        Self { 
            tempo_bpm,
            output_config: LiveOutputConfig { output_handle },
        }
    }

    pub fn play<T: Playable + Clone + Send + Sync + 'static>(&self, piece: T) -> std::thread::JoinHandle<()> {
        piece.play(self.output_config.output_handle.clone(), self.beat_duration_ms())
    }
}

impl MusicPlayer<FileOutputConfig> {
    pub fn new_file(tempo_bpm: u32, output_gain: f32, sample_rate: u32) -> Self {
        Self { 
            tempo_bpm,
            output_config: FileOutputConfig { output_gain, sample_rate }
        }
    }

    /* See render_to_wav.rs for implementation */
}

pub trait Playable {
    fn length(&self) -> usize;

    fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item=Note>;

    fn play(&self, output_handle: Arc<rodio::OutputStreamHandle>, beat_duration_ms: u64) -> JoinHandle<()>
        where Self: Send + Sync + Clone + 'static;
}

pub trait Pitchable {
    fn octave(&self, change: i32) -> Self
    where Self: Sized {
        self.semitone(change.saturating_mul(12))
    }

    fn semitone(&self, change: i32) -> Self
    where Self: Sized;
}

pub trait MusicOutput {}

#[derive(Clone)]
pub struct FileOutputConfig {
    /// Gain applied to the output audio (default: 1.0)
    output_gain: f32,
    /// Sample rate for audio generation (default: 44100 Hz)
    sample_rate: u32,
}

#[derive(Clone)]
pub struct LiveOutputConfig {
    output_handle: Arc<rodio::OutputStreamHandle>,
}

impl MusicOutput for FileOutputConfig {}

impl MusicOutput for LiveOutputConfig {}