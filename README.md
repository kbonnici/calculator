# Simple Calculator
Karl Bonnici

A very simple command-line based calculator application written in Rust.
Part of my [Projects to Learn Rust](https://github.com/kbonnici/projects-to-learn-rust) project collection.

## How to Run

```console
cargo run
```
Then input a simple expression to be evaluated.
## Examples
`5 + 5`, `27 / 3`

Currently, the only supported operators are:
* `+`
* `-`
* `*`
* `/`
and only integers/floats are supported as numbers.

## How to Test
```console
cargo test
```

## Todo
* [ ] Support for nested expressions. Example `1 + 2 / 3`, with operator precidence.
* [ ] Remember the most recently calculated value to use in the next expression
