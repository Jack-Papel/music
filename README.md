# Symphoxy

[![License](https://img.shields.io/crates/l/symphoxy)](https://github.com/Jack-Papel/symphoxy/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/symphoxy)](https://crates.io/crates/symphoxy)
[![Downloads](https://img.shields.io/crates/d/symphoxy)](https://crates.io/crates/symphoxy)

Symphoxy is a simple FOSS music-as-code library for rust.

## Goals

* **Flexible**: Easy to learn, but still powerful for more advanced use cases
* **Manipulatable**: Adding a note isn't carved in stone, you can remove it later in your code.
* **Quick**: Creating a song is fast, iterating is even faster.

## Contributing

Contributions are much appreciated. Feel free to open an issue or a pull request about any problems you encounter.

## Docs

Docs are sparse at the moment, but below is a quick-start guide.

### Trying it out

If you just want to try things out, you can clone this repository - there is a `main.rs` file that is already set up with everything you might need.

First, clone the repository:

```bash
git clone https://www.github.com/jack-papel/symphoxy
```

In [main.rs](https://github.com/Jack-Papel/symphoxy/blob/main/src/main.rs) there is a version of Mary Had a Little Lamb, and a song I presented for a final for one of my classes. By default, the main function will play the former. Play around with the code to see how to make changes and create music.

To play the music generated, run `cargo run`. This will immediately play the piece when the program is run. If instead, you want to be able to first edit configurations or write to a wav file, run with the `--features interactive-tui` flag. This will show a text user interface in your terminal that gives you more control over the song's playback.

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
  (verse_guitar() + chorus_quitar()) *
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
