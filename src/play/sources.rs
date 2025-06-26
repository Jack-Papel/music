use std::{io::BufReader, time::Duration, path::Path};

use rodio::{source::SineWave, Decoder, Source};

use crate::{note::Timbre, Tet12, C4};


pub fn get_source(duration_ms: u64, frequency: f32, timbre: Timbre, volume: f32) -> Box<dyn Source<Item=f32> + Send> {
    Box::new(get_dyn_source(duration_ms, frequency, timbre).amplify(volume))
}

fn get_dyn_source(duration_ms: u64, frequency: f32, timbre: Timbre) -> Box<dyn Source<Item=f32> + Send> {
    match timbre {
        Timbre::Sine => get_sine_source(duration_ms, frequency),
        Timbre::Bass => get_bass_source(duration_ms, frequency),
        Timbre::Piano => get_piano_source(duration_ms, frequency),
        Timbre::ElectricGuitar => get_electric_guitar_source(duration_ms, frequency),
        Timbre::Drums => get_drum_source(duration_ms, frequency),
    }
}

pub fn get_sine_source(duration_ms: u64, frequency: f32) -> Box<dyn Source<Item=f32> + Send> {
    let sources: Vec<Box<dyn Source<Item=f32> + Send>> = vec![
        Box::new(
            SineWave::new(frequency)
                .take_duration(Duration::from_millis(duration_ms.saturating_sub(40)))
                .fade_in(Duration::from_millis(40))
        ),
        Box::new(
            SineWave::new(frequency).fade_out(Duration::from_millis(40))
        )
    ];

    Box::new(rodio::source::from_iter(sources)
        .amplify((3.0 * 44.0 / frequency).clamp(0.0, 1.0)))
}

fn decibels_to_amplitude_ratio(dec: f32) -> f32 {
    10.0f32.powf(dec / 20.0)
}

pub fn get_drum_source(_duration_ms: u64, frequency: f32) -> Box<dyn Source<Item=f32> + Send> {
    fn read_drum_source_file(file: &str) -> Box<dyn Source<Item=f32> + Send> {
        let path = Path::new("src/assets").join(format!("{}.mp3", file));
        match std::fs::File::open(&path) {
            Ok(file) => {
                match Decoder::new(BufReader::new(file)) {
                    Ok(decoder) => Box::new(decoder.convert_samples().amplify(1.0)),
                    Err(_) => {
                        eprintln!("Warning: Could not decode audio file {:?}, using silence", path);
                        Box::new(rodio::source::Zero::<f32>::new(1, 44100).convert_samples().amplify(1.0))
                    }
                }
            },
            Err(_) => {
                eprintln!("Warning: Could not find drum file {:?}, using silence", path);
                Box::new(rodio::source::Zero::<f32>::new(1, 44100).convert_samples().amplify(1.0))
            }
        }
    }

    let kind = if frequency > C4.octave(1).semitone(6).0 {
        "crash"
    } else if frequency > C4.semitone(6).0 {
        "hi-hat"
    } else if frequency < C4.semitone(-6).0 {
        "kick"
    } else {
        "snare"
    };

    let base = read_drum_source_file(kind);
    if kind == "snare" {
        Box::new(base.amplify(5.0))
    } else {
        Box::new(base.amplify(2.5))
    }
}

pub fn get_electric_guitar_source(duration_ms: u64, frequency: f32) -> Box<dyn Source<Item=f32> + Send> {
    use rodio::source::SineWave;

    Box::new(
        SineWave::new(frequency)
        .mix(
            SineWave::new(frequency * 2.0)
                .amplify(decibels_to_amplitude_ratio(0.0))
        )
        .mix(
            SineWave::new(frequency * 3.0)
                .amplify(decibels_to_amplitude_ratio(8.0))
        )
        .mix(
            SineWave::new(frequency * 4.0)
                .amplify(decibels_to_amplitude_ratio(3.0))
        )
        .mix(
            SineWave::new(frequency * 5.0)
                .amplify(decibels_to_amplitude_ratio(-7.0))
        )
        .mix(
            SineWave::new(frequency * 6.0)
                .amplify(decibels_to_amplitude_ratio(-12.0))
        )
        .mix(
            SineWave::new(frequency * 7.0)
                .amplify(decibels_to_amplitude_ratio(-8.0))
        )
        .mix(
            SineWave::new(frequency * 8.0)
                .amplify(decibels_to_amplitude_ratio(-10.0))
        )
        .take_duration(Duration::from_millis(duration_ms))
        .amplify((3.0 * 44.0 / frequency).clamp(0.0, 1.0))
        .fade_out(Duration::from_millis(duration_ms))
    )
}

pub fn get_bass_source(duration_ms: u64, frequency: f32) -> Box<dyn Source<Item=f32> + Send> {
    use rodio::source::SineWave;

    Box::new(
        SineWave::new(frequency)
        .mix(
            SineWave::new(frequency * 2.0)
                .amplify(1.0 / 10.0)
        )
        .mix(
            SineWave::new(frequency * 3.0)
                .amplify(2.0)
        )
        .mix(
            SineWave::new(frequency * 4.0)
                .amplify(1.0 / 5.0)
        )
        .mix(
            SineWave::new(frequency * 5.0)
        )
        .mix(
            SineWave::new(frequency * 6.0)
        )
        .mix(
            SineWave::new(frequency * 7.0)
                .amplify(1.0 / 3.0)
        )
        .mix(
            SineWave::new(frequency * 8.0)
                .amplify(1.0 / 10.0)
        )
        .take_duration(Duration::from_millis(duration_ms))
        .amplify(12.0 * (3.0 * 44.0 / frequency).clamp(0.0, 1.0))
        .fade_out(Duration::from_millis(duration_ms))
    )
}

pub fn get_piano_source(duration_ms: u64, frequency: f32) -> Box<dyn Source<Item=f32> + Send> {
    use rodio::source::SineWave;

    Box::new(
        SineWave::new(frequency)
        .mix(
            SineWave::new(frequency * 2.0)
                .amplify(1.0 / 4.0)
        )
        .mix(
            SineWave::new(frequency * 3.0)
                .amplify(1.0 / 6.0)
        )
        .mix(
            SineWave::new(frequency * 4.0)
                .amplify(1.0 / 10.0)
        )
        .mix(
            SineWave::new(frequency * 5.0)
                .amplify(1.0 / 12.0)
        )
        .mix(
            SineWave::new(frequency * 6.0)
                .amplify(1.0 / 12.0)
        )
        .mix(
            SineWave::new(frequency * 7.0)
                .amplify(1.0 / 36.0)
        )
        .mix(
            SineWave::new(frequency * 8.0)
                .amplify(1.0 / 72.0)
        )
        .take_duration(Duration::from_millis(duration_ms))
        .amplify((12.0 * 44.0 / frequency).clamp(0.0, 1.0))
        .fade_in(Duration::from_millis(5))
        .fade_out(Duration::from_millis(duration_ms))
    )
}