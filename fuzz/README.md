# Fuzzing

Fuzz targets for `grain-id`, run with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) (requires nightly).

| Target      | What it checks |
|-------------|----------------|
| `parse`     | Parsing arbitrary strings as `GrainId` / `GrainIdPrefix` never panics, and parsed values round-trip through `Display`. |
| `roundtrip` | Every conversion of an arbitrary `GrainId` (integer, string, bytes, prefix) round-trips and agrees with the others. |
| `serde`     | Deserializing arbitrary JSON / postcard input never panics, and accepted values round-trip. |

```sh
cargo install cargo-fuzz
cd fuzz
cargo +nightly fuzz run parse -- -max_total_time=60
```

CI runs each target for 60 seconds on every pull request.
