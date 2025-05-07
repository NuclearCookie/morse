# Morse code translator crate
[![Crates.io](https://img.shields.io/crates/v/morse)](https://crates.io/crates/morse) [![Documentation](https://docs.rs/morse/badge.svg)](https://docs.rs/morse)

Provides functionality for encoding and decoding text to and from a morse code representation.

## Installation

### From Crates.io

```sh
cargo install morse
```

### From Source

```sh
git clone https://github.com/NuclearCookie/morse
cd morse
cargo install --path .
```

## Features
- The "norwegian" feature adds support for the norwegian letters "Æ", "Ø" and "Å" in accordance with the norwegian morse alphabet, a strict superset of the english morse code.
- The "spanish" feature adds support for the letters "ñ", "ü", some vowels, and accents