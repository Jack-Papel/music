# Symphoxy

[![CI](https://img.shields.io/github/actions/workflow/status/Jack-Papel/symphoxy/build.yml)](https://github.com/Jack-Papel/symphoxy/actions)
[![License](https://img.shields.io/crates/l/symphoxy)](https://github.com/Jack-Papel/symphoxy/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/symphoxy)](https://crates.io/crates/symphoxy)
[![Downloads](https://img.shields.io/crates/d/symphoxy)](https://crates.io/crates/symphoxy)
[![Docs](https://docs.rs/symphoxy/badge.svg)](https://docs.rs/symphoxy)
[![MSRV](https://img.shields.io/badge/MSRV-1.73.0-blue)](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1730-2023-10-05)

Symphoxy is a simple music-as-code library and synthesizer for rust.

## Goals

* **Flexible**: Easy to learn, but still powerful for more advanced use cases
* **Manipulatable**: Adding a note isn't carved in stone, you can remove it later in your code.
* **Quick**: Creating a song is fast, iterating is even faster.

## Contributing

Contributions are much appreciated. Feel free to open an issue or a pull request about any problems you encounter.

## Features

Symphoxy supports different output methods through feature flags:

* **`live-output`** (default): Play music in real-time using the system's audio output
* **`wav-output`** (default): Render music to WAV files
* **`interactive-tui`**: Provides an interactive terminal interface for playing or saving music

Enable features in your `Cargo.toml`:

```toml
[dependencies]
# Default features (live-output + wav-output)
symphoxy = "0.2"

# With interactive TUI
symphoxy = { version = "0.2", features = ["interactive-tui"] }

# Only WAV output (no live playback)
symphoxy = { version = "0.2", default-features = false, features = ["wav-output"] }
```

### Trying it out

If you just want to try things out, you can clone this repository - there are a few examples that are already set up.

First, clone the repository:

```bash
git clone https://www.github.com/jack-papel/symphoxy
```

In [the examples directory](https://github.com/Jack-Papel/symphoxy/tree/main/examples) there is a version of Mary Had a Little Lamb, and a song I presented for a final for one of my classes. Play around with the code to see how to make changes and create music.

To play the music generated, run `cargo run --example <example_name>`. This will launch an interactive TUI in your terminal that will allow you to either play the piece or save it to a wav file. If you want this capability when importing this crate into your own project, enable the `interactive-tui` feature in your Cargo.toml:

```toml
[dependencies]
symphoxy = { version = "*", features = "interactive-tui" }
```

If you want to integrate this crate in your own project, follow the quick start guide below.

### Getting started

First, import the symphoxy prelude to get all the tools you will need:

```rs
use symphoxy::prelude::*;
```

You can also import what you need on the fly.

A simple song looks like:

```rs
use symphoxy::prelude::*;

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
```

In order to hear the music you have created, do one of the following in your main function:

```rs
use std::sync::Arc;

use symphoxy::MusicPlayer;

// Get the default audio player using rodio
let (_output_stream, output_handle) = rodio::OutputStream::try_default().unwrap();
let output_handle = Arc::new(output_handle);

// Create a music player configuration
let player = MusicPlayer::new_live(300, output_handle);

// Get the song we created
let piece = mary_had_a_little_lamb().into();

// Play the piece and wait for it to end.
player.play(piece).join();
```

Or, if you would prefer to have more flexibility, enable the `interactive-tui` feature and do the following:

```rs
use symphoxy::InteractiveTui;

let piece = mary_had_a_little_lamb().into()

InteractiveTui::start(piece);
```

### The very basics

A piece consists of notes and lines. You can think of it like a table: Lines contain a list of notes, and pieces contain a list of lines which are played at the same time. Generally, the `+` operator adds things "to the right", in a sense - that is, it concatenates the operands. The `*` operator stacks things "downward", meaning connecting two things using `*` makes them play simultaneously. The `*` operator also multiplies things when the second operand is an integer.

Because of this, a rough outline of a verse and chorus might look like:

```rs
let verse_and_chorus = (
  (verse_vocals() + chorus_vocals()) *
  (verse_guitar() + chorus_guitar()) *
  bass_line() * 2 *
  drum_loop() * 2 *
);
```

or it could look like:

```rs
let verse = (
  verse_vocals() *
  verse_guitar() *
  bass_line() *
  drum_loop()
);

let chorus = (
  chorus_vocals() *
  chorus_guitar() *
  bass_line() *
  drum_loop()
);

let verse_and_chorus = verse + chorus
```

Because of this flexibility, it's easy to organize your song however makes the most sense to you.

## License

Symphoxy is free and open source software. See [LICENSE](https://github.com/Jack-Papel/symphoxy/blob/main/LICENSE) for usage information.
