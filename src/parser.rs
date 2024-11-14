use std::env;
#[derive(Clone, Copy, PartialEq, Debug)]

pub enum Opti   {
    All,
    List,
    Help,
    Version
}

pub fn parse() -> (Vec<String>, Vec<String>)  {
    let args = env::args().collect::<Vec<String>>();

    let mut options: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for arg in args {

        if arg.starts_with("--") == true    {
            options.push(arg);
        }
        else if arg.starts_with("-") == true {
            if arg.len() > 1    {
                let c = arg.split("").collect::<Vec<&str>>();
                for l in c  {
                    if l == "-" {
                        continue;
                    }
                    if l == ""  {
                        continue;
                    }
                    options.push(format!("-{}", l));
                }
            }
        }
        else  {
            values.push(arg);
        }
    }

    if values.len() == 1    {
        values.push(".".to_string());
    }

    (options, values)
}
