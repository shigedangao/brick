# Brick 🧱🍩

A proc-macro inspired from [struct_morph](https://github.com/shrynx/struct_morph/tree/main) to convert from a type using From or TryFrom for a `struct` or an `enum`

It supports these features:

- Field renaming
- Field exclusion by replacing with default value
- Error handling when using TryFrom trait
- Transformation func to convert a type to another

```rust
fn convert_ts_to_datetime(a: Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(a.seconds, 0).unwrap()
}

struct Source {
    name: String,
    ts: Timestamp,
}

#[brick(
    converter = "TryFrom",
    source = "Sourde",
    try_error_kind = "std::io::Error"
)]
struct Target {
    name: String,
    #[brick_field(transform_func = "convert_ts_to_datetime", rename = "ts")]
    timestamp: DateTime<Utc>,
    #[brick_field(exclude = true)]
    to_be_excluded: String,
}
```
