use std::{fs, path::PathBuf};

use crate::{
    entry::{
        kind::EntryKind,
        Entry,
        create
    },
    parser::Opti
};

enum TreeConnector   {
    Branch, // ├─
    LastBranch, // └──
    Vertical, // │
    Horizontal, // ─
    Empty //
}

impl TreeConnector  {
    fn as_str(&self) -> &'static str    {
        match self  {
            TreeConnector::Branch => "├─",
            TreeConnector::Vertical => "│",
            TreeConnector::Horizontal => "─",
            TreeConnector::LastBranch => "└──",
            TreeConnector::Empty => " "
        }
    }
}

// pub fn base(items: &[Entry], ops: Vec<Opti>) {
//     for (i, it) in items.iter().enumerate() {
//         if i == items.len() - 1   {
//             print!("{} ", TreeConnector::LastBranch.as_str());
//             println!("{}", it.name)
//         } else  {
//             print!("{} ", TreeConnector::Branch.as_str());
//             println!("{}", it.name);
//         }


//         if let EntryKind::Directory = it.entry_kind {
//             let mut pats: Vec<Entry> = Vec::new();
//             let mut p = PathBuf::new();
//             let pa = format!("./{}", it.name.clone());
//             let pads = fs::read_dir(pa).unwrap();
//             for pad in pads {
//                 p.push(pad.unwrap().file_name());
//                 // pats.push(create::filter_dir(&p).unwrap());
//                 // println!("pad: {:#?}", &pad..unwrap().path().display())
//                 if let Some(entry) = create::filter_dir(&p) {
//                     // pats.push(entry);
//                     println!("{:#?}", entry.name)
//                 }
//             }
//             println!("{:#?}", pats)
//         }
//     }
// }

pub fn base(items: &[Entry], ops: Vec<Opti>) {
    let mut w = 0;
    for (i, it) in items.iter().enumerate() {
        if i == items.len() - 1 {
            print!("{} ", TreeConnector::LastBranch.as_str());
            println!("{}", it.name)
        } else {
            print!("{}", TreeConnector::Branch.as_str());
            for a in 0..w   {
                print!("{}", TreeConnector::Horizontal.as_str())
            }
            println!(" {}", it.name);
        }

        if let EntryKind::Directory = it.entry_kind {
            let mut pats: Vec<Entry> = Vec::new();
            let pa = format!("{}", it.name.clone());
            match fs::read_dir(&pa) {
                Ok(pads) => {
                    for pad in pads {
                        // let path = pad.unwrap().path();  // Esto puede fallar, lo manejaremos después.
                        let path = pad.unwrap();  // Esto puede fallar, lo manejaremos después.
                        // match create::filter_dir(&path) {
                        match create::dir(&path, &ops) {
                            Some(entry) => pats.push(entry),
                            // None => eprintln!("No se pudo procesar el directorio: {}", path.display()),  // En vez de pánico, logueamos el error.
                            None => eprintln!("Could not process the directory: {:#?}", path),  // En vez de pánico, logueamos el error.
                        }
                    }
                    w += 1;
                    base(&pats, ops.clone());
                }
                Err(e) => eprintln!("Error reading the directory {}: {}", pa, e),  // Manejo de error si `read_dir` falla
            }
        }
    }
}
