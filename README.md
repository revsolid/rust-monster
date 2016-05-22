# rust-monster
A port of [GALib](http://lancet.mit.edu/ga/ "GA Lib") to rust-lang mainly to take advantage of parallelization.

## Note
This is a very early WIP.

## Building and Running Tests
rust-monster uses [cargo](https://crates.io/) packet manager and build tool-chain.

### Building
$> cargo build

### Runing Tests
$> cargo tests

### Running Tests with debug output
$>RUST_LOG=rust_monster=debug cargo tests 

