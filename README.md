# Anvil

A simple sha3 hashing cli that uses Rust's [sha3](https://docs.rs/sha3/latest/sha3/index.html) crate.

## Usage

```console
❯ anvil -h
Sha3 hashing cli.

Usage: anvil <METHOD> [VALUE]

Arguments:
  <METHOD>  [possible values: sha224, sha256, sha384, sha512, keccak224, keccak256, keccak384, keccak512]
  [VALUE]   The string value to hash. Can also be piped to stdin.

Options:
  -h, --help     Print help
  -V, --version  Print version
```

You can directly pass the string as a positional arg...
```console
❯ anvil sha256 'cow pig cat dog'
a1d80438bb2be9bae075db1e5be965fc301a52ca1bf046735b218ea7242ec8b9
```

Or pipe from stdin...

```console
❯ echo -n 'cow pig cat dog' | anvil sha256
a1d80438bb2be9bae075db1e5be965fc301a52ca1bf046735b218ea7242ec8b9
```

Which works with file contents as well...
```console
❯ cat farm-animals.txt | anvil sha256
a1d80438bb2be9bae075db1e5be965fc301a52ca1bf046735b218ea7242ec8b9
```