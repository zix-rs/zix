struct Cli  {
    name: String,
    version: String,
    help: String,
    flags: Vec<Flag>,
    commands: Vec<Command>,
    values: Vec<String>
}

struct Flag {
    name: String,
    value: String,
    long: String,
    short: String
}

struct Command  {
    flags: Vec<Flag>,
    values: Vec<String>
}
