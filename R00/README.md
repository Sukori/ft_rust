# R00
The only C function allowed in C is `write`. Based on this information, look for the equivalent in RUST and print numbers and letters.

## Observation
`let _ = std::io::stderr().write(format!("{}", c).as_bytes());` or `println!("{}", c);`?
In both cases, the write of different data types is possible.
However:
1. the `println!()` function is a builtin function of RUST
2. `stderr().write()` need the `use std::io::write;` call
3. `println!()` automatically prints a '\n' while `stderr().write()` doesn't

Common observation with ZIG is that the modern languages tend to remove at least this layer of complexity that C had.
