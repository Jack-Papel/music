use crate::Piece;

mod live_mode;
mod file_mode;

pub enum InteractiveCli {}

impl InteractiveCli {
    pub fn start(piece: Piece) {
        loop {
            let mode = InteractiveCli::get_input::<Mode>(());

            let result = match mode {
                Mode::Live => InteractiveCli::handle_live_mode(&piece),
                Mode::File => InteractiveCli::handle_file_mode(&piece)
            };

            match result {
                PlayResult::Exit => break,
                PlayResult::Continue => continue,
            }
        }

        println!("Exiting interactive mode.");
    }

    #[expect(clippy::arithmetic_side_effects, reason = "No selection will have usize::MAX options")]
    fn get_input<T: CliSelectable>(context: T::Context) -> T {
        let selections = T::get_selections(context);
        let options = selections.options;
        println!("{}:", selections.description);
        for (index, (key, _)) in options.iter().enumerate() {
            println!("    {}. {} ({})", index + 1, key.name, key.description);
        }
        if let Some(default) = selections.default {
            println!("Default: {}", options[default].0.name);
        }

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_lowercase();

            if input.is_empty() {
                if let Some(default) = selections.default {
                    return options[default].1;
                } else {
                    println!("Input cannot be empty, please try again.");
                    continue;
                }
            }

            if let Some((_, (_, value))) = options.iter().enumerate().find(|(idx, (selection, _))| {
                (idx + 1).to_string() == input || 
                selection.name.to_lowercase().starts_with(&input) || 
                selection.description.to_lowercase().starts_with(&input)
            }) {
                return *value;
            } else {
                println!("Invalid selection, please try again.");
            }
        }
    }

    fn get_range_input<const MIN: u32, const MAX: u32>(ask: &str) -> u32 {
        println!("{} (Between {} and {}):", ask, MIN, MAX);
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");

            if let Ok(value) = input.trim().parse() {
                if !(MIN..=MAX).contains(&value) {
                    println!("Please enter a value between {} and {}.", MIN, MAX);
                    continue;
                }
                return value;
            } else {
                println!("Invalid input. Please enter a valid BPM.");
                continue;
            }
        }
    }

    fn get_positive_float_input(ask: &str) -> f32 {
        println!("{} (Between 0.0 and infinity):", ask);
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");

            if let Ok(value) = input.trim().parse() {
                if value < 0.0 {
                    println!("Please enter a positive value.");
                    continue;
                }
                return value;
            } else {
                println!("Invalid input. Please enter a valid BPM.");
                continue;
            }
        }
    }
    
    fn get_path_input(ask: &str) -> String {
        println!("{}:", ask);
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_string();

            match Self::get_absolute_path(input.as_str()) {
                Ok(absolute_path) => {
                    return absolute_path;
                }
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            }
        }
    }

    fn get_absolute_path(path: &str) -> Result<String, String> {
        let path_input = std::path::Path::new(path);
        let Some(file_name) = path_input.file_name() else {
            return Err("Invalid path. Please enter a valid file name.".to_string());
        }; 
        let Some(parent) = path_input.parent() else {
            return Err("Failed to get parent directory. Please enter a valid path.".to_string());
        };
        let parent = if parent.as_os_str().is_empty() { std::path::Path::new(".") } else { parent };
        let Ok(absolute_parent_path) = parent.canonicalize() else {
            return Err("Failed to canonicalize path. Please enter a valid path.".to_string());
        };
        if !absolute_parent_path.exists() || !absolute_parent_path.is_dir() {
            return Err("Parent path is not a directory. Please enter a valid path.".to_string());
        }
        let Ok(output) = absolute_parent_path.join(file_name).into_os_string().into_string() else {
            return Err("Failed to convert path to string. Please enter a valid path.".to_string());
        };
        Ok(output)
    }
}

enum PlayResult {
    Continue,
    Exit,
}

trait CliSelectable: Sized + Copy {
    type Context;

    fn get_selections(context: Self::Context) -> Selections<Self>;
}

struct Selections<T> {
    pub description: String,
    pub default: Option<usize>,
    pub options: Vec<(SelectionInfo, T)>
}

struct SelectionInfo {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Copy)]
enum Mode {
    Live,
    File
}

impl CliSelectable for Mode {
    type Context = ();

    fn get_selections(_context: Self::Context) -> Selections<Self> {
        Selections {
            description: "Select an option".to_string(),
            default: None,
            options: vec![
                (SelectionInfo { name: "Play".to_string(), description: "Play music live".to_string() }, Mode::Live),
                (SelectionInfo { name: "Write".to_string(), description: "Render music to a WAV file".to_string() }, Mode::File),
            ]
        }
    }
}