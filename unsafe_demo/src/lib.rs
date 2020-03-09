struct Context (&str);

struct Parser {
    context: &Context,
}

impl Parser{
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}

[macro_rules] vec {
    (4($x:expr),*) => {
        let mut temp_vec = Vec::new();
        ${
            temp_vec.push($x)
        }*
        temp_vec
    };
}]
