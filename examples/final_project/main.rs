//! # Final Song
//! By Jack Papel
//! 
//! In this file is the song I created for my final using this library.
//! The lyrics aren't in here though.

use symphoxy::prelude::*;
use symphoxy::InteractiveTui;

// Moved to separate files for better organization,
// but you could inline them and still maintain a reasonable file size
mod bass;
mod drums;
mod guitar;
mod melody;

const BASS_VOL: f32 = 0.3;
const DRUM_VOL: f32 = 4.0;
const GUITAR_VOL: f32 = 2.0;
const MELODY_VOL: f32 = 8.0;

fn get_final_song() -> Piece {
    let intro = {
        bass::brain_stew(true).volume(BASS_VOL) * 2
        * (double_whole(REST) + drums::drums_verse() * 2).volume(DRUM_VOL)
    };

    let verse_1 = {
        (bass::brain_stew(true) * 4).volume(BASS_VOL)
        * (drums::drums_verse() * 7 + drums::drums_prechorus() + half(REST)).volume(DRUM_VOL)
        * (guitar::dings() * 3 + guitar::dings_prechorus()).volume(GUITAR_VOL)
        * (
            double_whole(REST) + melody::im_sad_your_back() + melody::and_i_need_a_big_mac() + 
            double_whole(REST) + melody::im_sad_your_back() + melody::tired_of_being_alone()
        ).volume(MELODY_VOL)
    };

    let chorus_1 = {
        bass::brain_stew(false).volume(BASS_VOL)
        * drums::groovy_drums().volume(DRUM_VOL)
        * guitar::groovy_dings_prechorus().volume(GUITAR_VOL)
    };

    let verse_2 = verse_1.clone();

    let chorus_2 = chorus_1.clone();

    let bridge = {
        bass::bridge_bass().volume(BASS_VOL)
        * drums::groovy_drums_alt().volume(DRUM_VOL)
        * guitar::groovy_dings().volume(GUITAR_VOL)
    } * 2;

    let verse_3 = verse_1.clone();

    let ending = {
        (bass::brain_stew(true) * 2).volume(BASS_VOL)
        * (drums::drums_verse() * 5).volume(DRUM_VOL)
        * (double_whole(REST) + melody::im_sad_your_back() + melody::tired_of_being_alone()).volume(MELODY_VOL)
    };

    intro + verse_1 + chorus_1 + verse_2 + chorus_2 + bridge + verse_3 + ending
}

fn main() {
    InteractiveTui::start(get_final_song());
}