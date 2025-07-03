use crate::{interactive::{TuiSelectable, InteractiveTui, PlayResult, SelectionInfo, Selections}, play::FileOutputConfig, MusicPlayer, Piece};


impl InteractiveTui {
    pub(super) fn handle_file_mode(piece: &Piece) -> PlayResult {
        let mut player = MusicPlayer::new_file(300, 1.0, 44100);
        let mut path = InteractiveTui::get_absolute_path("./output.wav");

        loop {
            let choice = InteractiveTui::get_input::<FileModeSelection>(FileModeSelectionContext {
                tempo: player.tempo_bpm,
                path: path.as_ref().ok().cloned(),
                output_config: player.output_config.clone(),
            });

            match choice {
                FileModeSelection::Render => {
                    if let Ok(ref path) = path.as_ref() {
                        println!("Rendering piece to {path}.");
                        player.render_to_wav(piece.clone(), path);
                        println!("Rendering complete. Saved to {path}.");
                    } else {
                        println!("No valid output path set. Please set a valid path first.");
                        continue;
                    }
                }
                FileModeSelection::ChangeTempo => {
                    let new_tempo = InteractiveTui::get_range_input::<10, 1000>("Enter tempo in BPM");
                    player.tempo_bpm = new_tempo;
                }
                FileModeSelection::ChangeOutputGain => {
                    let new_gain = InteractiveTui::get_positive_float_input("Enter output gain");
                    player.output_config.output_gain = new_gain;
                }
                FileModeSelection::ChangeSampleRate => {
                    let new_sample_rate = InteractiveTui::get_range_input::<8000, 192000>("Enter sample rate");
                    player.output_config.sample_rate = new_sample_rate;
                }
                FileModeSelection::ChangeOutputPath => {
                    let new_path = InteractiveTui::get_path_input("Enter output file path");
                    path = Ok(new_path);
                }
                FileModeSelection::Exit => return PlayResult::Exit,
                FileModeSelection::Continue => return PlayResult::Continue,
            }
        }
    }
}

#[derive(Clone, Copy)]
enum FileModeSelection {
    Render,
    ChangeTempo,
    ChangeOutputGain,
    ChangeSampleRate,
    ChangeOutputPath,
    Exit,
    Continue,
}

struct FileModeSelectionContext {
    tempo: u32,
    path: Option<String>,
    output_config: FileOutputConfig,
}

impl TuiSelectable for FileModeSelection {
    type Context = FileModeSelectionContext;

    fn get_selections(context: Self::Context) -> Selections<Self> {
        Selections {
            description: "File Mode Options".to_string(),
            options: vec![
                (SelectionInfo { name: "Write".to_string(), description: "Write the piece to a file".to_string() }, FileModeSelection::Render),
                (SelectionInfo { name: "Change Tempo".to_string(), description: format!("Current: {} BPM", context.tempo) }, FileModeSelection::ChangeTempo),
                (SelectionInfo { name: "Change Output Gain".to_string(), description: format!("Current: {}", context.output_config.output_gain) }, FileModeSelection::ChangeOutputGain),
                (SelectionInfo { name: "Change Sample Rate".to_string(), description: format!("Current: {} Hz", context.output_config.sample_rate) }, FileModeSelection::ChangeSampleRate),
                (SelectionInfo { name: "Change Output Path".to_string(), description: format!("Current: {}", if let Some(path) = context.path { path } else { "Unset".to_string() }) }, FileModeSelection::ChangeOutputPath),
                (SelectionInfo { name: "Exit".to_string(), description: "Leave interactive mode".to_string() }, FileModeSelection::Exit),
                (SelectionInfo { name: "Switch Mode".to_string(), description: "Return to mode selection".to_string() }, FileModeSelection::Continue),
            ],
            default: Some(0), // Default to Render
        }
    }
}