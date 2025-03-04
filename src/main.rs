//! # Final Song
//! By Jack Papel
//! 
//! In this file is the song I'm creating for my final using this software.
//! If you want to see documentation for how this library works, it is sparse, but there is a short explainer in lib.rs

use std::sync::Arc;
use music::{note::{note_length_fns::*, timbre_fns::*, NoteKind, NotePitch, REST}, scales::tet12::{map_semitones_to_pitches, C4}, Line, Piece, Playable};
use rodio::OutputStreamHandle;

fn brain_stew(fifth: bool) -> Piece {
    let g_sharp_3 = C4.semitone(8).octave(-2);
    const BASS_VOL: f32 = 0.3;

    fn downbeat(
        n1: NotePitch,
        n2: NotePitch,
        fifth: bool,
    ) -> Piece {
        let mut piece = 
        Piece::from(bass(eighth(n1).volume(BASS_VOL) + eighth(REST) * 3));

        if fifth {
            piece = piece * bass(eighth(n2).volume(BASS_VOL / 2.0) + eighth(REST) * 3)
        }

        piece
    }

    downbeat(g_sharp_3, g_sharp_3.semitone(7), fifth) +
    downbeat(g_sharp_3.semitone(-2), g_sharp_3.semitone(5), fifth) +
    downbeat(g_sharp_3.semitone(-3), g_sharp_3.semitone(4), fifth) +
    Piece::from(
        bass(eighth(g_sharp_3.semitone(-4)).volume(BASS_VOL) + eighth(REST) + eighth(g_sharp_3.semitone(-5)).volume(BASS_VOL) + eighth(REST))
    ) * (
        bass(eighth(g_sharp_3.semitone(3)).volume(BASS_VOL) + eighth(REST) + eighth(g_sharp_3.semitone(2)).volume(BASS_VOL) + eighth(REST))
    )
}

fn im_sad_your_back() -> Line {
    let c5 = C4.octave(1);
    piano(eighth(REST) + eighth(c5) + dotted(eighth)(c5.semitone(-2).into()) + sixteenth(REST)) +
    piano(eighth(REST) + eighth(c5.semitone(-2)) + dotted(eighth)(c5.semitone(-4).into()) + sixteenth(REST))
}

fn and_i_need_a_big_mac() -> Line {
    let c5 = C4.octave(1);
    piano(dotted(eighth)(REST) + sixteenth(c5.semitone(-9)) + eighth(c5.semitone(-2)) + sixteenth(c5.semitone(-2)) + sixteenth(c5)) +
    piano(eighth(c5.semitone(-2)) + eighth(c5.semitone(-4)) + quarter(REST))
}

fn tired_of_being_alone() -> Line {
    let c5 = C4.octave(1);
    -!piano(sixteenth(c5.semitone(-2)) + sixteenth(c5) + sixteenth(c5.semitone(1))) +
    piano(sixteenth(c5.semitone(1)) + sixteenth(c5) + sixteenth(c5.semitone(-2)) + eighth(c5.semitone(-4)) + sixteenth(c5.semitone(-9)) + sixteenth(c5.semitone(-2)) + sixteenth(c5.semitone(-4)) + eighth(c5.semitone(-4)))
}

fn wait_eight_beats() -> Line {
    tie(tie(half, half), tie(half, half))(REST).into()
}

fn from_guitar_frets(frets: [i32; 6]) -> [NotePitch; 6] {
    let (e_string, b_string, g_string, d_string, a_string, low_e_string) = (4, -1, -5, -10, -15, -20);

    map_semitones_to_pitches(C4, [e_string + frets[0], b_string + frets[1], g_string + frets[2], d_string + frets[3], a_string + frets[4], low_e_string + frets[5]])
}

const GUITAR_I: [i32;6] = [-1, 1, 1, 1, -1, -1];
const GUITAR_VI_M: [i32;6] = [1, 1, 1, 3, 3, 1];
const GUITAR_II_HALF_DIM: [i32;6] = [0, 2, 0, 1, 1, 0];
const GUITAR_V_7: [i32;6] = [-1, -1, 1, -1, -1, -1];

fn groove(note: impl Into<NoteKind> + Copy) -> Line {
    electric_guitar(eighth(REST) + eighth(note).volume(0.3) + (sixteenth(REST) + sixteenth(note).volume(0.3)) * 2)
}

fn chord_strike(note: impl Into<NoteKind> + Copy) -> Line {
    electric_guitar(eighth(REST) + sixteenth(note).volume(0.3) * 2)
}

fn dings() -> Piece {
    play_chord(from_guitar_frets(GUITAR_I), chord_strike) * 4 +
    play_chord(from_guitar_frets(GUITAR_VI_M), chord_strike) * 2 +
    play_chord(from_guitar_frets(GUITAR_II_HALF_DIM), chord_strike) +
    play_chord(from_guitar_frets(GUITAR_V_7), chord_strike)
}

fn play_chord(chord_notes: [impl Into<NoteKind> + Copy;6], striker: fn(NoteKind) -> Line) -> Piece {
    striker(chord_notes[0].into()) * 
    striker(chord_notes[1].into()) * 
    striker(chord_notes[2].into()) * 
    striker(chord_notes[3].into()) * 
    striker(chord_notes[4].into()) * 
    striker(chord_notes[5].into())
}
fn epic_chord_strike(note: impl Into<NoteKind> + Copy) -> Line {
    electric_guitar(quarter(note).volume(0.3))
}

#[expect(unused)]
fn get_final_song() -> Piece {
    let kick = C4.octave(-1);
    let snare = C4;
    let hi_hat = C4.octave(1);
    let crash = C4.octave(2);

    let intro = brain_stew(true) * 2 *
    (
        wait_eight_beats() + 
        (
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) + sixteenth(kick)) +
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) * 2)
        ) * 2
    );

    let verse_1 = brain_stew(true) * 4 *
    (
        (
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) + sixteenth(kick)) +
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) * 2)
        ) * 7 +
        drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + eighth(snare)) +
        half(REST)
    ) * 
    (
        (
            dings()
        ) * 3
        + 
        (
            play_chord(from_guitar_frets(GUITAR_I), chord_strike) * 4 +
            play_chord(from_guitar_frets(GUITAR_VI_M), chord_strike) * 2 +
            play_chord(from_guitar_frets(GUITAR_II_HALF_DIM), epic_chord_strike) + 
            play_chord(from_guitar_frets(GUITAR_V_7), epic_chord_strike)
        )
    ) *
    (
        wait_eight_beats() +
        im_sad_your_back() + and_i_need_a_big_mac() + 
        wait_eight_beats() + 
        im_sad_your_back() + tired_of_being_alone()
    );

    let groovy_drums = (
        drums(sixteenth(kick) + sixteenth(hi_hat) * 2 + (sixteenth(kick) + sixteenth(hi_hat)) * 2 + sixteenth(kick)) * 3 +
        drums(sixteenth(kick) + sixteenth(hi_hat) * 2 + sixteenth(kick) + (sixteenth(kick) + sixteenth(hi_hat)) * 2)
    );

    let groovy_drums_alt = (
        drums(sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(hi_hat) + sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(kick)) * 3 +
        drums(sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(kick)) * 2
    );

    let prechorus_1 = brain_stew(false) *
    groovy_drums *
    (
        play_chord(from_guitar_frets(GUITAR_I), groove) * 2 + 
        play_chord(from_guitar_frets(GUITAR_VI_M), groove) +
        play_chord(from_guitar_frets(GUITAR_II_HALF_DIM), epic_chord_strike) + 
        play_chord(from_guitar_frets(GUITAR_V_7), epic_chord_strike)
    );

    // TODO
    let chorus_1 = Piece::from(wait_eight_beats() * 2);
    let verse_2 = Piece::from(wait_eight_beats() * 2);
    let prechorus_2 = Piece::from(wait_eight_beats());
    let chorus_2 = Piece::from(wait_eight_beats() * 2);
    let bridge = Piece::from(wait_eight_beats() * 4);
    let verse_3 = Piece::from(wait_eight_beats() * 2);

    intro + verse_1 + prechorus_1
}

fn main() {
    let (_output_stream, output_stream_handle) = rodio::OutputStream::try_default().unwrap();
    let output_handle = Arc::new(output_stream_handle);

    showcase_and_play(get_final_song(), output_handle.clone());
}

fn showcase_and_play(piece: impl Into<Piece> + Clone, output_stream_handle: Arc<OutputStreamHandle>) {
    println!("{}", piece.clone().into().clone());

    let play_future = piece.into().play(output_stream_handle);
    let _ = play_future.join();
}
