# Examples

This directory contains example compositions demonstrating various features of Symphoxy.

## Running Examples

To run any example with the interactive TUI:

```bash
cargo run --example <example_name>
```

For example:
```bash
cargo run --example mary_had_a_little_lamb
```

## Available Examples

### `mary_had_a_little_lamb.rs`
A simple implementation of the classic children's song "Mary Had a Little Lamb". This example demonstrates:
- Basic note construction using scales
- Simple melody composition
- Use of the `piano` timbre
- Quarter notes and rests

### `final_project/`
A more complex musical composition showcasing advanced features:
- Multiple instruments and timbres
- Complex rhythmic patterns
- Chord progressions
- Multi-part arrangements

## Interactive Features

All examples support the interactive TUI when the `interactive-tui` feature is enabled, allowing you to:
- Play the composition in real-time
- Export to WAV files
- Adjust playback settings

Enjoy exploring the musical possibilities with Symphoxy! 🎵
