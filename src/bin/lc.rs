use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use structopt::StructOpt;
use stybulate::{Table, Style, Cell};

use linecounter::count_lines;


/// Print the number of lines for each file.
#[derive(StructOpt)]
struct Opt {
    /// The file(s) to read
    file: Vec<PathBuf>,
}

fn run(opt: Opt) -> std::io::Result<()> {
    if opt.file.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput,"Missing file!"))
    } else if opt.file.len() == 1 {
        let nb_lines = count_lines(&opt.file[0])?;
        println!("{}", nb_lines);
    } else {
        let mut results = vec![];
        for path in opt.file {
            let nb = count_lines(&path);
            match nb {
                Ok(n) => results.push(
                    vec![
                        Cell::from(&path.display().to_string()),
                        Cell::from(&n.to_string())
                    ]
                ),
                Err(e) => eprintln!("{}: {}", path.display(), e)
            };
        }
        let table = Table::new(Style::Plain,results, None);
        println!("{}", table.tabulate());
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("Error! {}", e);
    }
}
