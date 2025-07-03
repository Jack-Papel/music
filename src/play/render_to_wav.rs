//! AI wrote this, and I only mostly understand it.
//! Generally that's bad practice, but I determined it to be the best course of action because:
//! 1. Audio processing is complex and requires a lot of domain knowledge. I don't intend to study audio processing in depth. I just want WAV files.
//! 2. A drop-in solution doesn't exist in the Rust ecosystem that meets my needs.
//! 3. The code is tested and works correctly (for stereo, at least).
//!
//! This crate is open source, so if you find any issues with this code, or just want to make it simpler, please open an issue or PR.
//! This file is annotated with my best understanding of how it works.

#![allow(
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "Complex audio processing code"
)]
#![allow(clippy::needless_range_loop, clippy::needless_collect, reason = "Complex audio processing code")]

use std::ops::Div;

use crate::{
    play::{FileOutputConfig, Playable},
    MusicPlayer,
};

impl MusicPlayer<FileOutputConfig> {
    /// Renders a musical piece to a WAV file.
    ///
    /// This method generates audio samples for the entire piece and writes them
    /// to a WAV file at the specified path. The audio is rendered using the
    /// player's configured sample rate, gain, and tempo.
    ///
    /// # Arguments
    /// * `piece` - Any playable musical content (Note, Chord, Line, Piece, etc.)
    /// * `path` - The file path where the WAV file should be written
    ///
    /// # Example
    /// ```no_run
    /// use symphoxy::prelude::*;
    /// use symphoxy::MusicPlayer;
    ///
    /// let player = MusicPlayer::new_file(300, 1.0, 44100);
    /// let note = piano(quarter(C4));
    /// player.render_to_wav(note, "output.wav");
    /// ```
    ///
    /// # Panics
    /// This function panics if the file path is unable to be created or written to.
    #[expect(private_bounds, reason = "Only internal types should be playable")]
    pub fn render_to_wav<T: Playable + Clone + Send + Sync + 'static>(&self, piece: T, path: &str) {
        let FileOutputConfig {
            output_gain,
            sample_rate,
        } = self.output_config;

        let beat_duration_ms = self.beat_duration_ms();
        let length = piece.length();

        // Compute total duration in ms
        let total_ms = (length as u64).saturating_mul(beat_duration_ms);

        let total_samples: usize = (sample_rate as u64)
            .saturating_mul(total_ms)
            .div(1000)
            .try_into()
            .unwrap_or(usize::MAX);

        // Step 1: Find max channel count
        let mut max_channels = 1;

        // This could be more efficient if you made a Piece::get_all_notes() method,
        // but creating wav files doesn't take eons at the moment, so this is fine.
        for instant in 0..length {
            let notes: Vec<_> = piece.get_notes_at_instant(instant).collect();
            for note in notes {
                if let crate::note::NoteKind::Pitched { pitch, timbre, volume } = note.1 {
                    let duration_ms = (note.0 .0 as u64).saturating_mul(beat_duration_ms);
                    let frequency = pitch.0;
                    let src = super::sources::get_source(duration_ms, frequency, timbre, volume);
                    let native_channels = src.channels() as usize;
                    if native_channels > max_channels {
                        max_channels = native_channels;
                    }
                }
            }
        }

        // Allocate output buffers
        let mut samples: Vec<Vec<f32>> = vec![vec![0.0; total_samples]; max_channels];

        // Step 2: Render and mix
        for instant in 0..length {
            let notes: Vec<_> = piece.get_notes_at_instant(instant).collect();
            let start_ms = (instant as u64).saturating_mul(beat_duration_ms);
            for note in notes {
                match note.1 {
                    crate::note::NoteKind::Pitched { pitch, timbre, volume } => {
                        let duration_ms = (note.0 .0 as u64).saturating_mul(beat_duration_ms);
                        let frequency = pitch.0;
                        let src = super::sources::get_source(duration_ms, frequency, timbre, volume);
                        let native_sample_rate = src.sample_rate();
                        let native_channels = src.channels() as usize;

                        let note_samples = (sample_rate as u64)
                            .saturating_mul(duration_ms)
                            .div(1000)
                            .try_into()
                            .unwrap_or(usize::MAX);

                        let native_samples = (native_sample_rate as u64)
                            .saturating_mul(duration_ms)
                            .div(1000)
                            .try_into()
                            .unwrap_or(usize::MAX);

                        // Collect all channels
                        let mut chans: Vec<Vec<f32>> = vec![vec![]; native_channels];

                        // To my understanding, the samples are interleaved. That's why we do this
                        for (i, s) in src.take(native_samples * native_channels).enumerate() {
                            chans[i % native_channels].push(s);
                        }

                        // For each input channel, determine which output channel(s) to map to
                        for in_ch in 0..native_channels {
                            // Map input channel to output channel(s)
                            let out_ch = if native_channels == 1 {
                                // Mono: spread to all output channels
                                (0..max_channels).collect::<Vec<_>>()
                            } else {
                                // N-channel: map to proportional output channel
                                let idx = ((in_ch as f32) * (max_channels as f32 - 1.0)
                                    / (native_channels as f32 - 1.0))
                                    .round() as usize;
                                vec![idx]
                            };
                            let buf = if sample_rate != native_sample_rate {
                                // If you don't resample, the source will play slightly too fast / slow, causing pitch issues
                                resample_to_target_rate(
                                    chans[in_ch].clone().into_iter(),
                                    native_sample_rate,
                                    sample_rate,
                                    note_samples,
                                )
                            } else {
                                chans[in_ch].clone()
                            };

                            // Append all the samples to the output channels
                            let start_idx = (sample_rate as u64)
                                .saturating_mul(start_ms)
                                .div(1000)
                                .try_into()
                                .unwrap_or(usize::MAX);

                            for (i, &s) in buf.iter().enumerate() {
                                if let Some(idx) = start_idx.checked_add(i) {
                                    for &ch in &out_ch {
                                        if idx < samples[ch].len() {
                                            // For mono, divide by number of output channels to avoid boosting volume
                                            let val = if native_channels == 1 {
                                                s / max_channels as f32
                                            } else {
                                                s
                                            };
                                            samples[ch][idx] += val;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    crate::note::NoteKind::Rest => continue,
                }
            }
        }

        // Normalize all channels
        for ch in 0..max_channels {
            // It seems like this normalizes all channels separately, which seems strange but I trust the process.
            let max = samples[ch].iter().cloned().fold(0.0_f32, |a, b| a.abs().max(b.abs()));
            if max > 0.0 {
                for s in &mut samples[ch] {
                    *s = (*s / max) * output_gain;
                }
            }
        }

        // Write to WAV (interleaved)
        let spec = hound::WavSpec {
            channels: max_channels as u16,
            sample_rate,
            // This is apparently CD quality
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(path, spec).unwrap();

        // Convert to 16 bits per sample and int sample format
        for i in 0..total_samples {
            for ch in 0..max_channels {
                #[expect(clippy::cast_possible_truncation, reason = "It's clamped, so it should be safe")]
                let s: i16 = (samples[ch][i] * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
                writer.write_sample(s).unwrap();
            }
        }

        writer.finalize().unwrap();
    }
}

// This was originally a linear interpolation, but I changed it to cubic for better quality.
fn cubic_interp(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
    let a0 = y3 - y2 - y0 + y1;
    let a1 = y0 - y1 - a0;
    let a2 = y2 - y0;
    let a3 = y1;
    a0 * t * t * t + a1 * t * t + a2 * t + a3
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_wrap,
    reason = "Cubic interpolation and resampling require these conversions; safe for audio."
)]
// I assume this approximates inbetweening the samples using interpolation.
fn resample_to_target_rate<I: Iterator<Item = f32>>(
    input: I,
    input_rate: u32,
    output_rate: u32,
    num_samples: usize,
) -> Vec<f32> {
    if input_rate == output_rate {
        return input.take(num_samples).collect();
    }
    let input: Vec<f32> = input.collect();
    let input_len = input.len();
    let mut output = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f64 * (input_len as f64 - 1.0) / (num_samples as f64 - 1.0);
        let idx = t.floor() as isize;
        let frac = (t - idx as f64) as f32;
        // Get four points for cubic interpolation
        let y0 = *input.get((idx - 1).max(0) as usize).unwrap_or(&0.0);
        let y1 = *input.get(idx.max(0) as usize).unwrap_or(&0.0);
        let y2 = *input
            .get((idx + 1).min((input_len - 1) as isize) as usize)
            .unwrap_or(&0.0);
        let y3 = *input
            .get((idx + 2).min((input_len - 1) as isize) as usize)
            .unwrap_or(&0.0);
        output.push(cubic_interp(y0, y1, y2, y3, frac));
    }
    output
}
