# Brick 🧱

A proc-macro inspired from [struct_morph](https://github.com/shrynx/struct_morph/tree/main) to convert from a type using From or TryFrom for a `struct` or an `enum`

It supports these features:

- Field renaming
- Field exclusion by replacing with default value
- Error handling when using TryFrom trait
- Transformation func to perform any operation (either from a module, a trait or a function existing in the same scope)

## Basic sample

Below is an example of how to use the `brick` macro to perform a simple conversion from a type A to B and allowing to skip a field during the conversion.

```rust
use brick::brick;

struct Source {
    name: String,
}

#[brick(
    converter = "From",
)]
struct Target {
    name: String,
    #[brick_field(exclude = true)]
    bar: String
}
```

## Advanced example to process a field from source A to be inserted into Target struct while also renaming the field

```rust
fn convert_ts_to_datetime(a: Timestamp) -> Result<DateTime<Utc>, std::io::Error> {
    DateTime::from_timestamp(a.seconds, 0).ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to convert timestamp to datetime",
    ))
}

fn create_hello_world(a: String) -> String {
    format!("Hello {}", a)
}

struct Source {
    name: String,
    ts: Timestamp,
    hello: String,
}

#[brick(
    converter = "TryFrom",
    source = "Sourde",
    try_error_kind = "std::io::Error"
)]
struct Target {
    name: String,
    #[brick_field(transform_func = "convert_ts_to_datetime", rename = "ts", is_fallible = true)]
    timestamp: DateTime<Utc>,
    #[brick_field(transform_func = "create_hello_world")]
    hello: String,
}
```
