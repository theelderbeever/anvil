# Russet

A simple hashing cli that uses Rust's [sha3](https://docs.rs/sha3/latest/sha3/index.html) crate.

## Usage

```console
❯ russet -h
Hashing cli.

❯ echo -n 'some text' | russet sha256
802a5a961895b3f8c6556e31d0960a5778d7135be7d04bbbadd5e406c4bac381

❯ russet sha256 'some text'
n802a5a961895b3f8c6556e31d0960a5778d7135be7d04bbbadd5e406c4bac381

Usage: russet <METHOD> [VALUE]

Arguments:
  <METHOD>  [possible values: crc32, sha224, sha256, sha384, sha512, keccak224, keccak256, keccak384, keccak512]
  [VALUE]   The string value to hash. Can also be piped to stdin.

Options:
  -h, --help     Print help
  -V, --version  Print version
```

You can directly pass the string as a positional arg...
```console
❯ russet sha256 'cow pig cat dog'
a1d80438bb2be9bae075db1e5be965fc301a52ca1bf046735b218ea7242ec8b9
```

Or pipe from stdin...

```console
❯ echo -n 'cow pig cat dog' | russet sha256
a1d80438bb2be9bae075db1e5be965fc301a52ca1bf046735b218ea7242ec8b9
```

Which works with file contents as well...
```console
❯ cat farm-animals.txt | russet sha256
a1d80438bb2be9bae075db1e5be965fc301a52ca1bf046735b218ea7242ec8b9
```