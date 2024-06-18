use brick::brick;
use chrono::{DateTime, Utc};

fn lol(a: Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(a.seconds, 0).unwrap()
}

#[derive(Debug)]
#[brick(
    converter = "TryFrom",
    source_struct = "Bar",
    try_error_kind = "std::io::Error"
)]
struct Foo {
    name: String,
    #[brick_field(transform_func = "lol", rename = "ts")]
    timestamp: DateTime<Utc>,
}
struct Bar {
    name: String,
    ts: Timestamp,
}

struct Timestamp {
    seconds: i64,
}

enum Source {
    A,
}

#[derive(Debug)]
#[brick(converter = "From", source_enum = "Source")]
enum Target {
    A,
}

fn main() {
    let b = Bar {
        name: "Joe".to_string(),
        ts: Timestamp {
            seconds: 1717708136,
        },
    };

    let foo = Foo::try_from(b);

    dbg!(foo);

    let o = Source::A;
    let res = Target::from(o);

    dbg!(res);
}
