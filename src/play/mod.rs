#![expect(private_bounds, reason = "This is a public API, but the MusicOutput trait is private to prevent misuse")]

#[cfg(feature = "live-output")]
use std::{sync::Arc, thread::{self, JoinHandle}, time::Duration};

pub mod sources;
#[cfg(feature = "wav-output")]
mod render_to_wav;

#[cfg(feature = "live-output")]
use crate::{play::sources::get_source, NoteKind};

use crate::{Line, Note, Piece};


/// Creates a configuration for this music library
/// 
/// # Example
/// ```no_run
/// use symphoxy::MusicPlayer;
/// use symphoxy::prelude::*;
///
/// let piece = Piece::from(piano(quarter(C4) + quarter(A4)));
/// 
/// let player = MusicPlayer::new_file(300, 1.0, 44100);
/// 
/// player.render_to_wav(piece, "path/to/output.wav");
/// ```
pub struct MusicPlayer<O: MusicOutput + Clone> {
    /// Tempo in beats per minute (default: 300 BPM which gives 200ms per beat)
    pub(crate) tempo_bpm: u32,
    pub(crate) output_config: O,
}

impl<O: MusicOutput + Clone> MusicPlayer<O> {
    /// Calculate milliseconds per beat based on BPM
    pub(crate) fn beat_duration_ms(&self) -> u64 {
        60_000u64.checked_div(self.tempo_bpm as u64).unwrap_or(u64::MAX)
    }
}

#[cfg(feature = "live-output")]
impl MusicPlayer<LiveOutputConfig> {
    /// Creates a new music player for live audio output.
    /// 
    /// # Arguments
    /// * `tempo_bpm` - The tempo in beats per minute for playback. The number of sixteenth notes per minute.
    /// * `output_handle` - An Arc-wrapped rodio output stream handle for audio output
    /// 
    /// # Example
    /// ```no_run
    /// use symphoxy::MusicPlayer;
    /// use std::sync::Arc;
    /// 
    /// let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    /// let handle = Arc::new(handle);
    /// let player = MusicPlayer::new_live(300, handle);
    /// ```
    pub fn new_live(tempo_bpm: u32, output_handle: Arc<rodio::OutputStreamHandle>) -> Self {
        Self { 
            tempo_bpm,
            output_config: LiveOutputConfig { output_handle },
        }
    }

    /// Plays a musical piece through the live audio output.
    /// 
    /// This method spawns a background thread to handle audio playback and returns
    /// a join handle that can be used to wait for playback completion.
    /// 
    /// # Arguments
    /// * `piece` - Any playable musical content (Note, Chord, Line, Piece, etc.)
    /// 
    /// # Returns
    /// A `JoinHandle` that resolves when playback is complete
    /// 
    /// # Example
    /// ```no_run
    /// use symphoxy::prelude::*;
    /// use symphoxy::MusicPlayer;
    /// use std::sync::Arc;
    /// 
    /// let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    /// let handle = Arc::new(handle);
    /// let player = MusicPlayer::new_live(120, handle);
    /// let note = piano(quarter(C4));
    /// let handle = player.play(note);
    /// handle.join().unwrap(); // Wait for playback to finish
    /// ```
    pub fn play<T: Playable + Clone + Send + Sync + 'static>(&self, piece: T) -> std::thread::JoinHandle<()> {
        piece.play(self.output_config.output_handle.clone(), self.beat_duration_ms())
    }
}

impl MusicPlayer<FileOutputConfig> {
    /// Creates a new music player for file output (WAV rendering).
    /// 
    /// # Arguments
    /// * `tempo_bpm` - The tempo in beats per minute for the rendered audio
    /// * `output_gain` - The gain/volume multiplier for the output (1.0 = normal volume)
    /// * `sample_rate` - The sample rate in Hz for the output file (e.g., 44100)
    /// 
    /// # Example
    /// ```no_run
    /// use symphoxy::MusicPlayer;
    /// use symphoxy::prelude::*;
    ///
    /// let piece = Piece::from(piano(quarter(C4) + quarter(A4)));
    /// 
    /// let player = MusicPlayer::new_file(300, 1.0, 44100);
    /// 
    /// player.render_to_wav(piece, "path/to/output.wav");
    /// ```
    pub fn new_file(tempo_bpm: u32, output_gain: f32, sample_rate: u32) -> Self {
        Self { 
            tempo_bpm,
            output_config: FileOutputConfig { output_gain, sample_rate }
        }
    }

    /* See render_to_wav.rs for implementation */
}

trait MusicOutput {}

#[derive(Clone, Debug, PartialEq)]
pub struct FileOutputConfig {
    /// Gain applied to the output audio (default: 1.0)
    pub output_gain: f32,
    /// Sample rate for audio generation (default: 44100 Hz)
    pub sample_rate: u32,
}

#[derive(Clone)]
#[cfg(feature = "live-output")]
pub struct LiveOutputConfig {
    pub output_handle: Arc<rodio::OutputStreamHandle>,
}

#[cfg(feature = "wav-output")]
impl MusicOutput for FileOutputConfig {}

#[cfg(feature = "wav-output")]
impl Default for FileOutputConfig {
    fn default() -> Self {
        FileOutputConfig {
            output_gain: 1.0,
            sample_rate: 44100,
        }
    }
}

#[cfg(feature = "live-output")]
impl MusicOutput for LiveOutputConfig {}

pub(crate) trait Playable {
    #[cfg(feature = "wav-output")]
    fn length(&self) -> usize;

    #[cfg(feature = "wav-output")]
    fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item=Note>;

    #[cfg(feature = "live-output")]
    fn play(&self, output_handle: Arc<rodio::OutputStreamHandle>, beat_duration_ms: u64) -> JoinHandle<()>
        where Self: Send + Sync + Clone + 'static;
}

impl Playable for Piece {
    #[cfg(feature = "wav-output")]
    fn length(&self) -> usize {
        self.length()
    }

    #[cfg(feature = "wav-output")]
    fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item=Note> {
        self.get_notes_at_instant(instant)
    }

    #[cfg(feature = "live-output")]
    fn play(&self, output_handle: Arc<rodio::OutputStreamHandle>, beat_duration_ms: u64) -> JoinHandle<()> {
        let piece = self.clone();

        thread::spawn(move || {
            let mut handles = Vec::new();
            for instant in 0..piece.length() {
                for note in piece.get_notes_at_instant(instant) {
                    handles.push(note.play(output_handle.clone(), beat_duration_ms));
                }

                thread::sleep(Duration::from_millis(beat_duration_ms));
            }

            for handle in handles {
                let _ = handle.join();
            }
        })
    }
}

impl Playable for Line {
    /// Returns the length of this line without regard for the pickup
    #[cfg(feature = "wav-output")]
    fn length(&self) -> usize {
        self.length()
    }

    #[cfg(feature = "wav-output")]
    fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item=Note> {
        self.get_notes_at_instant(instant)
    }

    #[cfg(feature = "live-output")]
    fn play(&self, output_handle: Arc<rodio::OutputStreamHandle>, beat_duration_ms: u64) -> JoinHandle<()> {
        let line = self.clone();

        thread::spawn(move || {
            let mut handles = Vec::new();
            for instant in 0..line.length() {
                for note in line.get_notes_at_instant(instant) {
                    handles.push(note.play(output_handle.clone(), beat_duration_ms));
                }

                thread::sleep(Duration::from_millis(beat_duration_ms));
            }

            for handle in handles {
                let _ = handle.join();
            }
        })
    }
}

impl Playable for Note {
    #[cfg(feature = "wav-output")]
    fn length(&self) -> usize {
        self.0.0 as usize
    }

    #[cfg(feature = "wav-output")]
    fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item=Note> {
        if instant == 0 {
            Some(*self).into_iter()
        } else {
            None.into_iter()
        }
    }

    #[cfg(feature = "live-output")]
    fn play(&self, output_handle: Arc<rodio::OutputStreamHandle>, beat_duration_ms: u64) -> JoinHandle<()> {
        if let Note(length, NoteKind::Pitched { pitch, timbre, volume }) = *self {
            #[expect(clippy::arithmetic_side_effects, reason = "User's fault")]
            let duration_ms = length.0 as u64 * beat_duration_ms;

            thread::spawn(move || {
                let sink = rodio::Sink::try_new(&output_handle.clone()).unwrap();
                // For some reason, playing live is way louder than file output. 64 is arbitrary, but seems about right.
                sink.append(get_source(duration_ms, pitch.0, timbre, volume / 64.0));
                thread::sleep(Duration::from_millis(duration_ms));
                sink.sleep_until_end();
            })
        } else {
            thread::spawn(|| {})
        }
    }
}