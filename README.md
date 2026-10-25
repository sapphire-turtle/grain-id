# grain-id

<!-- cargo-rdme start -->

Compact, human-readable unique IDs — grain-sized, yet enough for a lifetime.

For a language agnostic specification of the grain-id format, see [SPECS.md](https://github.com/sapphire-turtle/grain-id/blob/main/SPECS.md)

## Quick Start
```rust
use grain_id::GrainId;

let id = GrainId::random();
println!("{}", id); // e.g. "123abcd"
```

## Why grain-id?

Traditional identifier systems face challenges in distributed environments:

- **Sequential numbers** (like GitHub issue numbers) cause collisions in distributed systems
- **UUIDs** are too long and not human-friendly
- **Short hashes** (like Git commit hashes) lack standardization

grain-id bridges the gap between human readability and technical requirements.

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
grain-id = "0.14.1"

# With optional features
grain-id = { version = "0.15.0", features = ["arbitrary", "serde", "schemars"] }
```

### For no_std Environments

This crate support `no_std`.
For `no_std` environment, you'll need to disable default features.
```toml
[dependencies]
grain-id = { version = "0.14.1", default-features = false }
```

## Features

- **Human-friendly**: Easy to read, type, and communicate
- **Collision-resistant**: Sufficient entropy for personal distributed systems
- **Compact**: Shorter than UUIDs while maintaining uniqueness
- **Type-safe**: Rust implementation with strong typing
- **Primitive-friendly**: Backed by a `u64`, so it maps directly onto integer columns and fields

### Optional Feature Flags

- `arbitrary`: `arbitrary::Arbitrary` support for fuzzing tests.
- `serde`: Serialization/deserialization support
- `schemars`: JSON Schema support

Integrations with databases, ORMs, or serialization formats beyond serde are intentionally not built in.
`GrainId` converts losslessly to and from `u64`, so you can store it as an integer column or field and convert at the boundary.
To derive an ID from a hash, pass the first 5 bytes of any digest to `GrainId::from_byte_prefix`.
If you would like a specific integration to be provided by this crate, please open an issue.

## Examples
```rust
use grain_id::GrainId;
// Generate random grain-id
let grain_id = GrainId::random();

// e.g. `123abcd`
println!("'{}'", grain_id);

// Parse from string
let valid_id: GrainId = "012atvw".parse()?;

// When decoding from BASE32, ambiguous characters (1/l/I, 0/o, v/u) are treated as 1, 0 and v respectively, so they do not cause errors.
let also_valid_id: GrainId = "ol2atuw".parse()?;
assert_eq!(valid_id, also_valid_id);

// Convert to/from integer
let num: u64 = valid_id.into();
let id_from_int: GrainId = num.try_into()?;
assert_eq!(valid_id, id_from_int);

// Lossy conversion from oversized int is allowed.
let id_from_overflowed_int = GrainId::from_u64_lossy(GrainId::CAPACITY + num);
assert_eq!(valid_id, id_from_overflowed_int);

```

<!-- cargo-rdme end -->

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
