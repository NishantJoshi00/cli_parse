pub enum ArgType { // This is for storing the place holders Retrived after Parsing
    Int(i32),
    Str(String),
    None,
}

enum ArgType_ { // Private variable for templating
    Int,
    Str,
    None
}

struct Argument_ { // Private variable for templating
    name: String,
    nickname: Option<String>,
    value: ArgType_
}

pub struct Argument { // Struct containing arguments that are parsed and resolved
    pub name: String,
    pub value: ArgType
}

pub struct Parsed { // Final output of the CLI_PARSER
    pub binary: String,
    pub arguments: Vec<Argument>,
    pub command: Option<String>
}


fn get_type(name: String, template: &Vec<Argument_>, nick: bool) -> (Option<ArgType_>, String) {
    // This is a hidden function used to parse the arguments and return Them in the form of tuple
    for i in template {
        if nick {
            match &i.nickname {
                Some(val) => if val.clone() == name {
                    match &i.value {
                        ArgType_::Int => return (Some(ArgType_::Int), i.name.clone()),
                        ArgType_::Str => return (Some(ArgType_::Str), i.name.clone()),
                        ArgType_::None => return (Some(ArgType_::None), i.name.clone())
                    }
                },
                None => {}
            }
        } else {
            if i.name.clone() == name {
                match &i.value {
                    ArgType_::Int => return (Some(ArgType_::Int), i.name.clone()),
                    ArgType_::Str => return (Some(ArgType_::Str), i.name.clone()),
                    ArgType_::None => return (Some(ArgType_::None), i.name.clone())
                }
            }
        }
    }

    (None, name.clone())
}


fn from_tuple(template: Vec<(&str, Option<&str>, &str)>) -> Vec<Argument_> {
    // This for making templating easier for end developer
    let mut output: Vec<Argument_> = Vec::new();
    for i in template {
        output.push(Argument_ {
            name: i.0.to_owned(),
            nickname: match i.1 {
                Some(v) => Some(v.to_owned()),
                None => None
            },
            value: if i.2.to_lowercase() == "int" {
                ArgType_::Int
            } else if i.2.to_lowercase() == "str" {
                ArgType_::Str
            } else {
                ArgType_::None
            }
        })
    }
    output
}
