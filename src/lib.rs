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