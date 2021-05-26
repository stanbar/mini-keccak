# mini-Keccak

[mini-Keccak hash function](https://en.wikipedia.org/wiki/SHA-3) written in Rust.

## Usage

To calculate hash of some input you can either pass it as an argument

```
cargo run <some input>
```

or pass as an stdin

```
echo -n <some input> | cargo run
```


## Example

Finding the reverses of some hashes

```
cargo run --bin reverse-hash --release
```
