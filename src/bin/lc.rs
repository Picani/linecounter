use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use structopt::StructOpt;
use stybulate::{Cell, Style, Table};

use linecounter::count_lines;

/// Print the number of lines for each file.
#[derive(StructOpt)]
struct Opt {
    /// The file(s) to read
    #[structopt(required=true)]
    file: Vec<PathBuf>,
}

fn run(opt: Opt) -> std::io::Result<()> {
    if opt.file.len() == 1 {
        let nb_lines = count_lines(&opt.file[0])?;
        println!("{}", nb_lines);
    } else {
        let mut results = vec![];
        for path in opt.file {
            let nb = count_lines(&path);
            match nb {
                Ok(n) => results.push(vec![
                    Cell::from(&path.display().to_string()),
                    Cell::from(&n.to_string()),
                ]),
                Err(e) => eprintln!("{}: {}", path.display(), e),
            };
        }
        let table = Table::new(Style::Plain, results, None);
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
