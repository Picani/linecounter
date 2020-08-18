linecounter
===========

A `wc -l` like tool that works with compressed files.

__Note:__ currently support `deflate`, `zip`, `gzip`.

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
    lc [file]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file>...    The file(s) to read
```
