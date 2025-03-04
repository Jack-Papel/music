use crate::note::NotePitch;

use super::Scale;

pub fn get_note_name(note: NotePitch, a4: NotePitch) -> String {
    let c4 = a4.semitone(3).octave(-1);

    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

    let diff = f32::log2(note.0 / c4.0);
    let (octave_diff, semitone_diff) = (diff.round() as i16, ((diff * 12.0).round() as i16).rem_euclid(12));

    String::from(note_names[semitone_diff as usize]) + &(octave_diff + 4).to_string()
}

pub fn map_semitones_to_pitches<const N: usize>(root: NotePitch, semitones: [i32;N]) -> [NotePitch;N] {
    semitones.map(|s| root.semitone(s))
}

pub struct MajorScale(pub NotePitch);

pub const A4: NotePitch = NotePitch(440.0);
pub const C4: NotePitch = NotePitch(261.626);

impl Scale for MajorScale {
    fn get_pitch(&self, degree: isize) -> NotePitch {
        let adjusted_degree = if degree > 0 {
            degree - 1
        } else {
            degree
        };
        let pattern = [2, 2, 1, 2, 2, 2, 1];
        let octave_power = adjusted_degree.div_euclid(7) as f32;

        let mut interval_power = 0.0f32;
        for i in 0..adjusted_degree.rem_euclid(7) {
            interval_power += (pattern[i as usize] as f32) / 12.0
        }

        NotePitch(self.0.0 * 2.0f32.powf(octave_power + interval_power))
    }
}