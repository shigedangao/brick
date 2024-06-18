use brick::brick;
use chrono::{DateTime, Utc};

fn convert_ts_to_datetime(a: Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(a.seconds, 0).unwrap()
}

#[derive(Debug)]
#[brick(
    converter = "TryFrom",
    source_struct = "Source",
    try_error_kind = "std::io::Error"
)]
struct Target {
    name: String,
    #[brick_field(transform_func = "convert_ts_to_datetime", rename = "ts")]
    timestamp: DateTime<Utc>,
}

struct Source {
    name: String,
    ts: Timestamp,
}

struct Timestamp {
    seconds: i64,
}

enum SourceEnum {
    A,
}

#[derive(Debug)]
#[brick(converter = "From", source_enum = "SourceEnum")]
enum TargetEnum {
    A,
}

fn main() {
    let b = Source {
        name: "Dodoooo".to_string(),
        ts: Timestamp {
            seconds: 1717708136,
        },
    };

    let foo = Target::try_from(b);

    dbg!(foo);

    let o = SourceEnum::A;
    let res = TargetEnum::from(o);

    dbg!(res);
}
