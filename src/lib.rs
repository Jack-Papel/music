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


pub mod piece;
pub mod note;
pub mod scales;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub use piece::Piece;
pub use piece::line::Line;
pub use note::Note;

pub trait Playable {
    fn length(&self) -> usize;

    fn get_notes_at_instant(&self, instant: usize) -> impl Iterator<Item=Note>;

    fn play(&self, output_handle: Arc<rodio::OutputStreamHandle>) -> JoinHandle<()>
        where Self: Send + Sync + Clone + 'static
    {
        let piece = self.clone();

        thread::spawn(move || {
            let mut handles = Vec::new();
            for instant in 0..piece.length() {
                for note in piece.get_notes_at_instant(instant) {
                    handles.push(note.play(output_handle.clone()));
                }
    
                thread::sleep(Duration::from_millis(200));
            }

            for handle in handles {
                let _ = handle.join();
            }
        })
    }
}

pub trait Pitchable {
    fn octave(&self, change: i32) -> Self
    where Self: Sized {
        self.semitone(change * 12)
    }

    fn semitone(&self, change: i32) -> Self
    where Self: Sized;
}