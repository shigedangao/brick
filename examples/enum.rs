use brick::brick;

fn process_enum_content(msg: String) -> TargetEnum {
    TargetEnum::Oddo(format!("hello, 你好， ສະບາຍດີ {msg}"))
}
enum SourceEnum {
    Foo,
    #[allow(dead_code)]
    Bar,
    Nado(String),
    Naming {
        firstname: String,
        lastname: String,
    },
}

#[derive(Debug)]
#[brick(converter = "From", source = "SourceEnum")]
enum TargetEnum {
    Foo,
    #[brick_field(rename = "Bar")]
    B,
    #[brick_field(rename = "Nado", transform_func = "process_enum_content")]
    Oddo(String),
    #[brick_field(rename = "Naming")]
    Name {
        firstname: String,
        lastname: String,
    },
}

fn main() {
    let src = SourceEnum::Foo;
    let res = TargetEnum::from(src);

    let src_with_nado = SourceEnum::Nado("world".to_string());
    let res_with_nado = TargetEnum::from(src_with_nado);

    if let TargetEnum::Oddo(msg) = res_with_nado {
        assert_eq!(msg, "hello, 你好， ສະບາຍດີ world");
        println!("{}", msg);
    }

    println!("{:?}", res);
}
