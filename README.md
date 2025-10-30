# Brick 🧱

A proc-macro inspired from [struct_morph](https://github.com/shrynx/struct_morph/tree/main) to convert from a type using From or TryFrom for a `struct` or an `enum`

It supports these features:

- Field renaming
- Field exclusion by replacing with default value
- Error handling when using TryFrom trait
- Transformation func to perform any operation (either from a module, a trait or a function existing in the same scope)
- IsFallible support when using TryFrom trait to mark a transformation function as fallible (Result sum type)

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

## Enum conversion example

Enum is more complex to work with than the struct. Below are the features that are supported:

- Field renaming
- Error handling when using TryFrom trait
- Transformation func to perform any operation (either from a module, a trait or a function existing in the same scope)

## Basic example

Below is an example of how to use the `brick` macro to perform a conversion of a Source enum to a PayloadFromSource enum. Here the goal is to convert an enum which contains some Error and map to our own defined Error

```rust
// Raw error codes from a database or external system
enum DbError {
    NotFound(String),
    Timeout(u32),
    InvalidData(String),
    ConnectionLost(String),
}

// User-friendly error messages
#[derive(Debug)]
#[brick(converter = "From", source = "DbError")]
enum UserError {
    #[brick_field(rename = "NotFound", transform_func = "format_not_found")]
    NotFound(String),

    #[brick_field(rename = "Timeout", transform_func = "format_timeout")]
    Timeout(String),

    #[brick_field(rename = "InvalidData", transform_func = "format_invalid")]
    ValidationError(String),

    #[brick_field(rename = "ConnectionLost", transform_func = "format_connection")]
    NetworkError(String),
}

fn format_not_found(table: String) -> UserError {
    UserError::NotFound(format!("❌ Could not find record in {}", table))
}

fn format_timeout(seconds: u32) -> UserError {
    UserError::Timeout(format!("⏱️  Request timed out after {} seconds", seconds))
}

fn format_invalid(reason: String) -> UserError {
    UserError::ValidationError(format!("⚠️  Invalid data: {}", reason))
}

fn format_connection(details: String) -> UserError {
    UserError::NetworkError(format!("🔌 Connection issue: {}", details))
}

fn main() {
    // Database returns raw error
    let db_err = DbError::NotFound("users".to_string());
    let user_err: UserError = db_err.into();
    println!("{:?}", user_err);

    // Timeout example
    let timeout = DbError::Timeout(30);
    let user_timeout = UserError::from(timeout);
    println!("{:?}", user_timeout);

    // Validation error
    let invalid = DbError::InvalidData("email format incorrect".to_string());
    if let UserError::ValidationError(msg) = UserError::from(invalid) {
        println!("{}", msg);
    }
}
```
