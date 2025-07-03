# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-07-02

### Added

- Added "Chord" type
- Added tools for stringed (e.g. guitar) chords and tunings
- Comprehensive documentation and examples for all public APIs
- Added more standard library trait implementations for ergonomics
- Display implementations for debugging
- Added constructor methods to manually-constructable types
- Support for array-based construction of chords, lines, and pieces

### Fixed

- All clippy warnings and missing documentation
- Doc test failures across the codebase
- More types can be passed into timbre functions (e.g. `piano`, `bass`)
- More types can be passed into note length functions (e.g. `quarter`, `half`)

## [0.1.0] - Initial Release

### Added

- Core music composition types (Note, Line, Piece)
- Musical scales and tuning systems (12-tone equal temperament)
- Instrument tools (guitar fretting, string tunings)
- Live audio output via rodio
- WAV file export capabilities
- Interactive terminal UI for playback and export
- Multiple built-in timbres and note lengths
- Arithmetic operations on musical structures
