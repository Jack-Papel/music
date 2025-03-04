use std::{io::BufReader, time::Duration};

use rodio::{source::{SamplesConverter, SineWave}, Decoder, Source};

use crate::scales::tet12::C4;

use super::Timbre;

const ONE_BEAT_MS: u64 = 200;


fn get_dyn_source(length: u16, frequency: f32, timbre: Timbre) -> Box<dyn Source<Item=f32> + Send> {
    match timbre {
        Timbre::Sine => Box::new(get_sine_source(length, frequency)),
        Timbre::Bass => Box::new(get_bass_source(length, frequency)),
        Timbre::Piano => Box::new(get_piano_source(length, frequency)),
        Timbre::ElectricGuitar => Box::new(get_electric_guitar_source(length, frequency)),
        Timbre::Drums => Box::new(get_drum_source(length, frequency)),
    }
}

pub fn get_source(length: u16, frequency: f32, timbre: Timbre, volume: f32) -> Box<dyn Source<Item=f32> + Send> {
    Box::new(get_dyn_source(length, frequency, timbre).amplify(volume))
}

pub fn get_sine_source(length: u16, frequency: f32) -> impl Source<Item=f32> {
    let sources: Vec<Box<dyn Source<Item=f32> + Send>> = vec![
        Box::new(
            SineWave::new(frequency)
                .take_duration(Duration::from_millis(length as u64 * ONE_BEAT_MS - 40))
                .fade_in(Duration::from_millis(40))
        ),
        Box::new(
            SineWave::new(frequency).fade_out(Duration::from_millis(40))
        )
    ];

    rodio::source::from_iter(sources)
        .amplify((3.0 * 44.0 / frequency).clamp(0.0, 1.0))
}

fn decibels_to_amplitude_ratio(dec: f32) -> f32 {
    10.0f32.powf(dec / 20.0)
}

pub fn get_drum_source(_length: u16, frequency: f32) -> impl Source<Item=f32> {
    fn read_drum_source_file(file: &str) -> SamplesConverter<Decoder<BufReader<std::fs::File>>, f32> {
        Decoder::new(
            BufReader::new(std::fs::File::open(String::new() + "src/assets/" + file + ".mp3").unwrap())
        ).unwrap().convert_samples()
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

    if kind == "snare" {
        read_drum_source_file(kind).amplify(5.0)
    } else {
        read_drum_source_file(kind).amplify(2.5)
    }
}

pub fn get_electric_guitar_source(length: u16, frequency: f32) -> impl Source<Item=f32> {
    use rodio::source::SineWave;

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
    .take_duration(Duration::from_millis(length as u64 * ONE_BEAT_MS))
    .amplify((3.0 * 44.0 / frequency).clamp(0.0, 1.0))
    .fade_out(Duration::from_millis(length as u64 * ONE_BEAT_MS))
}

pub fn get_bass_source(length: u16, frequency: f32) -> impl Source<Item=f32> {
    use rodio::source::SineWave;

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
    .take_duration(Duration::from_millis(length as u64 * ONE_BEAT_MS))
    .amplify(12.0 * (3.0 * 44.0 / frequency).clamp(0.0, 1.0))
    .fade_out(Duration::from_millis(length as u64 * ONE_BEAT_MS))
}

pub fn get_piano_source(length: u16, frequency: f32) -> impl Source<Item=f32> {
    use rodio::source::SineWave;

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
    .take_duration(Duration::from_millis(length as u64 * ONE_BEAT_MS))
    .amplify((12.0 * 44.0 / frequency).clamp(0.0, 1.0))
    .fade_in(Duration::from_millis(5))
    .fade_out(Duration::from_millis(length as u64 * ONE_BEAT_MS))
}