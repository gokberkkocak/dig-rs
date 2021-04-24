# Dominance Incomparability Generator ![build/test](https://github.com/gokberkkocak/dig-rs/actions/workflows/ci.yml/badge.svg)

## Build

```rust
cargo build --release
```

## Usage

```
dig 0.2.0
Dominance Incomparability Generator

USAGE:
    dig-rs [FLAGS] --input <input> --output <output>

FLAGS:
    -b, --bdd        Simplify via bdd instead of laws
    -g, --graph      Generate dot plot
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Input Essence model
    -o, --output <output>    Output json file
```

Information about its usage is in [dig.md](dig.md).
