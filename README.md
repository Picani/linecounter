linecounter
===========

[![dependency status](https://deps.rs/repo/github/picani/linecounter/status.svg)](https://deps.rs/repo/github/picani/linecounter)

An easy to use and fast tool that counts the number of lines from files.

Think of it as `wc -l` and `grep -c` merged in a single tool with compressed
file support. It also doesn't count lines on unsupported files format
(because yes, `wc -l` gives you the number of text lines in a video), and
nicely formats the output.  

For now, it's already faster than things like `zcat myfile.gz | wc -l` or
`zgrep -c '>' myHugeFastaFile.gz`. However, it's *way* slower than a simple
 `wc -l` or `grep -c`. Help's needed to figure out why!

Supported compression format:

* `gzip`

Installation
------------

Build with 

`$ cargo build --release`

The executable is `target/release/lc`.

Usage
-----

```shell script
linecounter 0.1.0
Print the number of lines for each file

USAGE:
    lc [FLAGS] [OPTIONS] <file>...

FLAGS:
    -h, --help       Prints help information
    -t, --trim       Remove leading whitespace before to look for the prefix
    -V, --version    Prints version information

OPTIONS:
    -p, --prefix <prefix>    Count only the lines starting with that prefix

ARGS:
    <file>...    The file(s) to read
```
