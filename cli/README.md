# grain-id-cli
Reference tool to generate/encode/decode [grain-id](https://github.com/fluo10/grain-id).

## Installation

```
cargo install grain-id-cli
```

## Usage

```
Reference tool to generate/encode/decode grain-id

Usage: grain-id <COMMAND>

Commands:
  decode     Decode grain-id string to integer
  encode     Encode integer to grain-id string
  generate   (deprecated) Generate random grain-id
  timestamp  Generate time-based grain-id
  random     Generate random grain-id
  md5        Generate grain-id from MD5 hash of input
  sha1       Generate grain-id from SHA-1 hash of input
  sha256     Generate grain-id from SHA-256 hash of input
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Generate new GrainIdS

```
$ grain-id random
123abcd
```

### Generate time-based GrainId

```
$ grain-id timestamp --unix
gdw0982
```

### Encode GrainIdD

```
$ grain-id encode 0
0000000
```

### Decode GrainIdQ

```
$ grain-id decode 0000000
0
```

### Generate GrainId from hash

```
$ grain-id md5 "hello world"
btv3qez
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
