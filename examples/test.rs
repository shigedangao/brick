use brick::{brick, brick_field};
use chrono::{DateTime, Utc};

fn lol(a: Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(a.seconds, 0).unwrap()
}

#[derive(Debug)]
#[brick(converter = "From", source_struct = "Bar")]
struct Foo {
    name: String,
    #[brick_field(convert_field_func = "lol")]
    ts: DateTime<Utc>
}

struct Bar {
    name: String,
    ts: Timestamp
}

struct Timestamp {
    seconds: i64
}

fn main() {
    let b = Bar {
        name: "Joe".to_string(),
        ts: Timestamp {
            seconds: 1717708136
        }
    };

    let foo = Foo::from(b);
    dbg!(foo);
}
