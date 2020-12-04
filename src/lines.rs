use std::io::BufRead;

pub enum CountParameters {
    All,
    KeepPrefix(String),
}

/// Count the number of lines in *input*, using *parameters* to choose
/// which lines to take into account.
///
/// Return that number or an error if the data are not valid UTF-8.
pub fn nb_lines(
    input: impl BufRead,
    parameters: &CountParameters
) -> std::io::Result<usize> {
    match parameters {
        CountParameters::All => count_lines(input),
        CountParameters::KeepPrefix(prefix) => count_lines_keep_prefix(input, &prefix),
    }
}


/// Count the number of lines in *input*.
///
/// Return it or an error if the data are not valid UTF-8.
fn count_lines(input: impl BufRead) -> std::io::Result<usize> {
    let mut res = 0;

    for line in input.lines() {
        match line {
            Ok(_) => res += 1,
            Err(e) => return Err(e)
        }
    }

    Ok(res)
}

/// Count the number of lines in *input*, keeping only lines starting with
/// *prefix*.
///
/// Return the number of kept lines or an error if the data are not valid
/// UTF-8.
fn count_lines_keep_prefix(
    input: impl BufRead,
    prefix: &str
) -> std::io::Result<usize> {
    let mut res  = 0;

    for line in input.lines() {
        match line {
            Ok(l) => if l.starts_with(prefix) { res += 1; },
            Err(e) => return Err(e)
        }
    }

    Ok(res)
}

