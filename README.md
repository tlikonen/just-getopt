Just Getopt
===========

**A getopt-like command-line parser for the Rust language**

Introduction
------------

This [Rust][] language library crate implements a Posix `getopt`-like
command-line option parser with simple programming interface. More
specifically the parser is like `getopt`’s GNU extension called
`getopt_long` which is familiar command-line option format for users of
Linux-based operating systems.

The name is *Just Getopt* because this is *just a getopt parser* and
(almost) nothing more. The intent is to provide just the parsed output
and methods for examining the output. There will not be anything for
interpreting the output or for printing messages to program’s user. The
responsibility of interpretation is left to your program.

In getopt logic there are two types of command-line options:

 1. short options with a single letter name (`-f`)
 2. long options with more than one letter as their name (`--file`).

Both option types may accept an optional value or they may require a
value. Values are given after the option.


[Rust]: https://www.rust-lang.org/


Usage and Documentation
-----------------------

The crate is available at [Github][] and [crates.io][] and it can added
to a Rust project with command `cargo add just-getopt`.

Documentation is available at [docs.rs][]. From the source code
directory the documentation can be built and shown in a web browser with
command `cargo doc --open`.

Also see file [examples/basic.rs](examples/basic.rs) for basic
programming examples.


[Github]:    https://github.com/tlikonen/just-getopt
[crates.io]: https://crates.io/crates/just-getopt
[docs.rs]:   https://docs.rs/just-getopt/


License
-------

Author: Teemu Likonen <<tlikonen@iki.fi>>

OpenPGP key: [6965F03973F0D4CA22B9410F0F2CAE0E07608462][PGP]

License: [Creative Commons CC0][CC0] (public domain dedication)

[PGP]: http://www.iki.fi/tlikonen/pgp-key.asc
[CC0]: https://creativecommons.org/publicdomain/zero/1.0/legalcode
