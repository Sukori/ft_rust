# R00
The only C function allowed in C is `write`. Based on this information, look for the equivalent in RUST and print numbers and letters.

## Observation
`let _ = std::io::stderr().write(format!("{}", c).as_bytes());` or `println!("{}", c);`?
In both cases, the write of different data types is possible. Therefore, it seems unnecessary to use a long version, as of now.

Common observation with ZIG is that the modern languages tend to remove at least this layer of complexity that C had.
