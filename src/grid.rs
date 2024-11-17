pub mod prgrid  {
    use crate::app::Entry;
    use crate::parser::Opti;
    use crate::window::get_terminal_size;
    use unicode_width::UnicodeWidthStr;

    pub enum Format {
        List,
        Base
    }

    pub fn list(ops: Vec<Opti>, items: Vec<Entry>)   {
        for entry in items.iter()   {
            if ops.contains(&Opti::List)  {
                println!(
                    "{:<6} \t {:<19} {:>8} {}",
                    entry.mode, entry.last_modified, entry.lenght, entry.name
                );
            }
        }
    }

    pub fn base(ve: Vec<String>)  {
        let max_width = ve
        .iter()
        .map(|name| UnicodeWidthStr::width(name.as_str()))
        .max()
        .unwrap_or(0)
        + 2;

        let (term_width, _) = get_terminal_size();

            println!("{}", term_width);
        let w =  if term_width >= 80 {
                        <u16 as Into<usize>>::into(term_width) - 20
                        } else {
                            <u16 as Into<usize>>::into(term_width)
                        };
        let columns = w / max_width;

        for (i, name) in ve.iter().enumerate() {
            print!("{:<width$}", name, width = max_width);
            if (i + 1) % columns == 0 {
                println!();
            }
        }
        println!();
    }
}
