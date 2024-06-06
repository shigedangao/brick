use brick::brick;

#[brick(converter = "From", source_struct = "Bar")]
struct Foo {
    name: String,
}

struct Bar {
    name: String,
}

fn main() {
    let f = Foo {
        name: "Joe".to_string(),
    };
}
