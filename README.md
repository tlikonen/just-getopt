Just Getop
==========

**A getopt-like command-line parser for the Rust language**

Introduction
------------

This [Rust][] language library crate implements a Posix `getopt`-like
command-line option parser with simple programming interface. More
specifically the parser is like `getopt`’s GNU extension called
`getopt_long` which is familiar command-line option format for users of
Linux-based operating systems.

The name is `just_getopt` because this is *just a getopt parser* and
(almost) nothing more. The intent is to provide just the parsed output
and methods for examining the output. There will not be anything for
interpreting the output or for printing messages to program’s user. The
responsibility of interpretation is left to your program.

In getopt logic there are two types of command-line options:

 1. short options with a single letter name (`-f`)
 2. long options with more than one letter as their name (`--file`).

Both option types may accept an optional value or they may require a
value. Values are given after the option.

Documentation of this library crate is available in the source file
([`src/lib.rs`](src/lib.rs)). It can be built and shown in a web browser
with command `cargo doc --open`. That requires the usual Rust
development tools.

See file [`examples/basic.rs`](examples/basic.rs) for basic programming
examples.


[Rust]: https://www.rust-lang.org/


License and Source Code
-----------------------

Author: Teemu Likonen <<tlikonen@iki.fi>>

OpenPGP key: [6965F03973F0D4CA22B9410F0F2CAE0E07608462][PGP]

License: [Creative Commons CC0][CC0] (public domain dedication)

The source code repository: <https://github.com/tlikonen/just-getopt>


[PGP]: http://www.iki.fi/tlikonen/pgp-key.asc
[CC0]: https://creativecommons.org/publicdomain/zero/1.0/legalcode
