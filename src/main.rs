//! # Final Song
//! By Jack Papel
//! 
//! In this file is the song I created for my final using this software.
//! If you want to see documentation for how this library works, it is sparse, but there is a short explanation in lib.rs
//! and a more detailed explanation in the README.md file.
//! 
//! If you want to test the interactive TUI, you can run this file with the `interactive-tui` feature enabled.

use symphoxy::prelude::*;
#[cfg(feature = "interactive-tui")]
use symphoxy::InteractiveTui;

const BASS_VOL: f32 = 0.3;
const DRUM_VOL: f32 = 4.0;
const GUITAR_VOL: f32 = 2.0;
const MELODY_VOL: f32 = 8.0;
fn downbeat_bass(
    n1: NotePitch,
    n2: NotePitch,
    fifth: bool,
) -> Piece {
    let mut piece = 
    Piece::from(bass(eighth(n1) + eighth(REST) * 3));

    if fifth {
        piece = piece * bass(eighth(n2) + eighth(REST) * 3)
    }

    piece
}

fn brain_stew(fifth: bool) -> Piece {
    let g_sharp_3 = C4.semitone(8).octave(-2);

    downbeat_bass(g_sharp_3, g_sharp_3.semitone(7), fifth) +
    downbeat_bass(g_sharp_3.semitone(-2), g_sharp_3.semitone(5), fifth) +
    downbeat_bass(g_sharp_3.semitone(-3), g_sharp_3.semitone(4), fifth) +
    Piece::from(
        bass(eighth(g_sharp_3.semitone(-4)) + eighth(REST) + eighth(g_sharp_3.semitone(-5)) + eighth(REST))
    ) * (
        bass(eighth(g_sharp_3.semitone(3)) + eighth(REST) + eighth(g_sharp_3.semitone(2)) + eighth(REST))
    )
}

fn bridge_bass() -> Piece {
    let g_sharp_3 = C4.semitone(8).octave(-2);

    downbeat_bass(g_sharp_3, g_sharp_3.semitone(7), true) +
    downbeat_bass(g_sharp_3.semitone(1), g_sharp_3.semitone(8), true) +
    downbeat_bass(g_sharp_3.semitone(2), g_sharp_3.semitone(9), true) +
    downbeat_bass(g_sharp_3.semitone(4), g_sharp_3.semitone(11), true)
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

fn from_guitar_frets(frets: [i16; 6]) -> [NotePitch; 6] {
    let (e_string, b_string, g_string, d_string, a_string, low_e_string) = (4, -1, -5, -10, -15, -20);

    [
        e_string + frets[0],
        b_string + frets[1],
        g_string + frets[2],
        d_string + frets[3],
        a_string + frets[4],
        low_e_string + frets[5]
    ].map(|semitone| C4.semitone(semitone))
}

const GUITAR_I: [i16;6] = [-1, 1, 1, 1, -1, -1];
const GUITAR_VI: [i16;6] = [1, 2, 2, 3, 3, 1];
const GUITAR_VI_M: [i16;6] = [1, 1, 1, 3, 3, 1];
const GUITAR_II_HALF_DIM: [i16;6] = [0, 2, 0, 1, 1, 0];
const GUITAR_V_7: [i16;6] = [-1, -1, 1, -1, -1, -1];

fn groove(note: impl Into<NoteKind> + Copy) -> Line {
    electric_guitar(eighth(REST) + eighth(note) + (sixteenth(REST) + sixteenth(note)) * 2)
}

fn chord_strike(note: impl Into<NoteKind> + Copy) -> Line {
    electric_guitar(eighth(REST) + sixteenth(note) * 2)
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
    electric_guitar(quarter(note))
}

#[expect(unused)]
fn get_final_song() -> Piece {
    let kick = C4.octave(-1);
    let snare = C4;
    let hi_hat = C4.octave(1);
    let crash = C4.octave(2);

    let intro = brain_stew(true).volume(BASS_VOL) * 2 *
    (
        wait_eight_beats() + 
        (
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) + sixteenth(kick)) +
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) * 2)
        ) * 2
    ).volume(DRUM_VOL);

    let verse_1 = brain_stew(true).volume(BASS_VOL) * 4 *
    (
        (
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) + sixteenth(kick)) +
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) * 2)
        ) * 7 +
        drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + eighth(snare)) +
        half(REST)
    ).volume(DRUM_VOL) * 
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
    ).volume(GUITAR_VOL) *
    (
        wait_eight_beats() +
        im_sad_your_back() + and_i_need_a_big_mac() + 
        wait_eight_beats() + 
        im_sad_your_back() + tired_of_being_alone()
    ).volume(MELODY_VOL);

    let groovy_drums = (
        drums(sixteenth(kick) + sixteenth(hi_hat) * 2 + (sixteenth(kick) + sixteenth(hi_hat)) * 2 + sixteenth(kick)) * 3 +
        drums(sixteenth(kick) + sixteenth(hi_hat) * 2 + sixteenth(kick) + (sixteenth(kick) + sixteenth(hi_hat)) * 2)
    );

    let groovy_drums_alt = (
        drums(sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(hi_hat) + sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(kick)) * 3 +
        drums(sixteenth(kick) + sixteenth(hi_hat) + sixteenth(snare) + sixteenth(kick)) * 2
    );

    let prechorus_1 = brain_stew(false).volume(BASS_VOL) *
    groovy_drums.volume(DRUM_VOL) *
    (
        play_chord(from_guitar_frets(GUITAR_I), groove) * 2 + 
        play_chord(from_guitar_frets(GUITAR_VI_M), groove) +
        play_chord(from_guitar_frets(GUITAR_II_HALF_DIM), epic_chord_strike) + 
        play_chord(from_guitar_frets(GUITAR_V_7), epic_chord_strike)
    ).volume(GUITAR_VOL);

    // TODO
    let chorus_1 = Piece::from(wait_eight_beats() * 2);
    let verse_2 = verse_1.clone();
    let prechorus_2 = prechorus_1.clone();
    let chorus_2 = Piece::from(wait_eight_beats() * 2);
    let bridge = (
        bridge_bass()
    ).volume(BASS_VOL) * (
        groovy_drums_alt.volume(DRUM_VOL)
    ) * (
        play_chord(from_guitar_frets(GUITAR_I), groove) + 
        play_chord(from_guitar_frets(GUITAR_VI), groove) +
        play_chord(from_guitar_frets(GUITAR_II_HALF_DIM), groove) + 
        play_chord(from_guitar_frets(GUITAR_V_7), groove)
    ).volume(GUITAR_VOL) * 2;
    let verse_3 = verse_1.clone();
    let ending = brain_stew(true).volume(BASS_VOL) * 2 *
    (
        (
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) + sixteenth(kick)) +
            drums(eighth(kick) + eighth(snare) + sixteenth(REST) + sixteenth(kick) + sixteenth(snare) * 2)
        ) * 5
    ).volume(DRUM_VOL) * (
        wait_eight_beats() + 
        im_sad_your_back() + tired_of_being_alone()
    ).volume(MELODY_VOL);
        

    intro + verse_1 + prechorus_1 + verse_2 + prechorus_2 + bridge + verse_3 + ending
}

fn mary_had_a_little_lamb() -> impl Into<Piece> {
    let c_major = symphoxy::scales::MajorScale(C4);
    let [c4, d4, e4, g4] = c_major.get_degrees([1, 2, 3, 5]);
    
    piano(
        quarter(e4) + quarter(d4) + quarter(c4) + quarter(d4) +
        quarter(e4) * 3 + quarter(REST) +
        quarter(d4) * 3 + quarter(REST) +
        quarter(e4) + quarter(g4) * 2 + quarter(REST) +
        quarter(e4) + quarter(d4) + quarter(c4) + quarter(d4) +
        quarter(e4) * 4 + quarter(d4) * 2 +
        quarter(e4) + quarter(d4) + quarter(c4) + quarter(REST)
        + quarter(c4.octave(1))
    )
}

fn main() {
    #[cfg(feature = "interactive-tui")]
    InteractiveTui::start(mary_had_a_little_lamb().into());
    #[cfg(not(feature = "interactive-tui"))]
    {
        use std::sync::Arc;

        use symphoxy::MusicPlayer;

        let (_output_stream, output_handle) = rodio::OutputStream::try_default().unwrap();
        let output_handle = Arc::new(output_handle);

        let player = MusicPlayer::new_live(300, output_handle);

        player.play(mary_had_a_little_lamb().into()).join();
    }
}
