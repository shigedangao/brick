use brick::brick;

fn process_enum_content(msg: String) -> TargetEnum {
    TargetEnum::Oddo(format!("hello, 你好， ສະບາຍດີ {msg}"))
}

fn process_enum_source(src_enum: SourceEnum) -> TargetEnum {
    match src_enum {
        SourceEnum::Foo => TargetEnum::Foo,
        _ => TargetEnum::B,
    }
}

fn process_origin_content(country: String, city: String) -> TargetEnum {
    TargetEnum::Origin {
        country: country.to_uppercase(),
        city: city.to_uppercase(),
    }
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
    Origin {
        country: String,
        city: String,
    },
}

#[derive(Debug)]
#[brick(converter = "From", source = "SourceEnum")]
enum TargetEnum {
    Foo,
    #[brick_field(rename = "Bar")]
    B,
    #[brick_field(rename = "Bar", transform_func = "process_enum_source")]
    #[allow(unused)]
    C,
    #[brick_field(rename = "Nado", transform_func = "process_enum_content")]
    Oddo(String),
    #[brick_field(rename = "Naming")]
    Name {
        firstname: String,
        lastname: String,
    },
    #[brick_field(transform_func = "process_origin_content")]
    Origin {
        country: String,
        city: String,
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

    let src_origin = SourceEnum::Origin {
        country: "laos".to_string(),
        city: "luang Prabang".to_string(),
    };
    let tgt_origin = TargetEnum::from(src_origin);
    if let TargetEnum::Origin { country, city } = tgt_origin {
        assert_eq!(country, "LAOS");
        assert_eq!(city, "LUANG PRABANG");
    }

    let tgt_name = SourceEnum::Naming {
        firstname: "Nado".into(),
        lastname: "Dodo".into(),
    };
    let res_name = TargetEnum::from(tgt_name);
    if let TargetEnum::Name {
        firstname,
        lastname,
    } = res_name
    {
        assert_eq!(firstname, "Nado");
        assert_eq!(lastname, "Dodo");
    }

    println!("{:?}", res);
}
