use brick::brick;
use chrono::{DateTime, Utc};

// A dummy module to show that we can use a function from another module
mod utils {
    pub fn append_hello(input: String) -> String {
        format!("Hello, {}", input)
    }
}

// Convert a timestamp to a chrono DateTime
fn convert_ts_to_datetime(a: Timestamp) -> Result<DateTime<Utc>, std::io::Error> {
    DateTime::from_timestamp(a.seconds, 0).ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to convert timestamp to datetime",
    ))
}

struct Timestamp {
    seconds: i64,
}

struct Source {
    name: String,
    ts: Timestamp,
    hello: String,
}

#[derive(Debug)]
#[brick(
    converter = "TryFrom",
    source = "Source",
    try_error_kind = "std::io::Error"
)]
struct Target {
    #[allow(dead_code)]
    name: String,
    #[brick_field(
        transform_func = "convert_ts_to_datetime",
        rename = "ts",
        is_fallible = true
    )]
    #[allow(dead_code)]
    timestamp: DateTime<Utc>,
    #[brick_field(exclude = true)]
    #[allow(dead_code)]
    excluded: bool,
    #[brick_field(transform_func = "append_hello", fn_from_extern = "utils")]
    hello: String,
}

fn main() {
    let b = Source {
        name: "Doudou".to_string(),
        ts: Timestamp {
            seconds: 1717708136,
        },
        hello: "doudou".to_string(),
    };

    let foo = Target::try_from(b);
    assert_eq!(foo.unwrap().hello, "Hello, doudou");
}
