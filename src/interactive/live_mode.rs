use std::sync::Arc;

use crate::{
    interactive::{InteractiveTui, PlayResult, SelectionInfo, Selections, TuiSelectable},
    MusicPlayer, Piece,
};

impl InteractiveTui {
    pub(super) fn handle_live_mode(piece: &Piece) -> PlayResult {
        let Ok((_output_stream, output_handle)) = rodio::OutputStream::try_default() else {
            println!("Failed to get default output stream. Please ensure your audio output is configured correctly.");
            return PlayResult::Continue;
        };
        let output_handle = Arc::new(output_handle);

        let mut player = MusicPlayer::new_live(300, output_handle);
        let mut show_score = false;
        loop {
            let choice = InteractiveTui::get_input::<LiveModeSelection>(LiveModeSelectionContext {
                show_score,
                tempo: player.tempo_bpm as u64,
            });

            match choice {
                LiveModeSelection::ChangeTempo => {
                    let new_tempo = InteractiveTui::get_range_input::<10, 1000>("Enter tempo in BPM");

                    player.tempo_bpm = new_tempo;
                    println!("Tempo changed to {new_tempo} BPM.");
                }
                LiveModeSelection::Play => {
                    if show_score {
                        println!("Playing piece with score display:");
                        println!("{piece}");
                    } else {
                        println!("Playing piece without score display.");
                    }
                    player.play(piece.clone()).join().expect("Failed to play piece");
                }
                LiveModeSelection::ToggleScore => {
                    show_score = !show_score;
                }
                LiveModeSelection::Exit => return PlayResult::Exit,
                LiveModeSelection::Continue => return PlayResult::Continue,
            }
        }
    }
}

#[derive(Clone, Copy)]
enum LiveModeSelection {
    ChangeTempo,
    ToggleScore,
    Play,
    Exit,
    Continue,
}

struct LiveModeSelectionContext {
    show_score: bool,
    tempo: u64,
}

impl TuiSelectable for LiveModeSelection {
    type Context = LiveModeSelectionContext;

    fn get_selections(context: Self::Context) -> Selections<Self> {
        Selections {
            description: "Live Mode Options".to_string(),
            options: vec![
                (
                    SelectionInfo {
                        name: "Play".to_string(),
                        description: "Perform the current piece".to_string(),
                    },
                    Self::Play,
                ),
                (
                    SelectionInfo {
                        name: "Change Tempo".to_string(),
                        description: format!("Current: {} BPM", context.tempo),
                    },
                    Self::ChangeTempo,
                ),
                (
                    SelectionInfo {
                        name: if context.show_score { "Hide Score" } else { "Show Score" }.to_string(),
                        description: "Toggle score display".to_string(),
                    },
                    Self::ToggleScore,
                ),
                (
                    SelectionInfo {
                        name: "Exit".to_string(),
                        description: "Leave interactive mode".to_string(),
                    },
                    Self::Exit,
                ),
                (
                    SelectionInfo {
                        name: "Switch Mode".to_string(),
                        description: "Return to mode selection".to_string(),
                    },
                    Self::Continue,
                ),
            ],
            default: Some(0),
        }
    }
}
