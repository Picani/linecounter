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
}

fn run(opt: Opt) -> std::io::Result<()> {
    let lines_param = CountParameters::All;

    if opt.file.len() == 1 {
        let f = open(&opt.file[0])?;
        let nb = nb_lines(f, &lines_param)?;
        println!("{}", nb);
    } else {
        let mut results = vec![];
        for path in opt.file {
            let f = open(&path)?;
            let nb = nb_lines(f, &lines_param);
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
