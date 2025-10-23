# Brick 🧱

A proc-macro inspired from [struct_morph](https://github.com/shrynx/struct_morph/tree/main) to convert from a type using From or TryFrom for the targeted struct

It also support renaming field

```rust
fn convert_ts_to_datetime(a: Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(a.seconds, 0).unwrap()
}

#[brick(
    converter = "TryFrom",
    source_struct = "Bar",
    try_error_kind = "std::io::Error"
)]
struct Foo {
    name: String,
    #[brick_field(transform_func = "convert_ts_to_datetime", rename = "ts")]
    timestamp: DateTime<Utc>,
}
struct Bar {
    name: String,
    ts: Timestamp,
}
```
