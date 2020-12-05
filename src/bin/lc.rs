use std::path::PathBuf;

use structopt::StructOpt;
use structopt::clap::AppSettings;
use stybulate::{Cell, Style, Table};

use linecounter::{CountParameters, nb_lines, open};

/// Print the number of lines for each file.
#[derive(StructOpt)]
#[structopt(settings = &[AppSettings::ColoredHelp])]
struct Opt {
    /// The file(s) to read
    #[structopt(required=true)]
    file: Vec<PathBuf>,

    /// Count only the lines starting with that prefix
    #[structopt(short = "p", long = "prefix")]
    prefix: Option<String>,

    /// Remove leading whitespace before to look for the prefix
    #[structopt(short = "t", long = "trim", requires = "prefix")]
    trim: bool,
}

fn run(opt: Opt) -> std::io::Result<()> {
    let lines_param = if let Some(prefix) = opt.prefix {
        CountParameters::KeepPrefix(prefix, opt.trim)
    } else {
        CountParameters::All
    };

    if opt.file.len() == 1 {
        let f = open(&opt.file[0])?;
        let nb = nb_lines(f, &lines_param)?;
        println!("{}", nb);
    } else {
        let mut results = vec![];
        for path in opt.file {
            let nb;
            match open(&path) {
                Ok(f) => {
                    nb = nb_lines(f, &lines_param);
                },
                Err(e) => {
                    eprintln!("{}: {}", path.display(), e);
                    continue;
                }
            }

            match nb {
                Ok(n) => {
                    results.push(vec![
                        Cell::from(&path.display().to_string()),
                        Cell::from(&n.to_string()),
                    ]);
                },
                Err(e) => {
                    eprintln!("{}: {}", path.display(), e);
                }
            }
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
