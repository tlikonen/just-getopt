//! # Introduction
//!
//! This library crate implements a Posix `getopt`-like command-line
//! option parser with simple programming interface. More specifically
//! the parser is like `getopt`'s GNU extension called `getopt_long`
//! which is familiar command-line option format for users of
//! Linux-based operating systems.
//!
//! The name is `just_getopt` because this is *just a getopt parser* and
//! (almost) nothing more. The intent is to provide just the parsed
//! output and methods for examining the output. There will not be
//! anything for interpreting the output or for printing messages to
//! program's user. The responsibility of interpretation is left to your
//! program.
//!
//! In getopt logic there are two types of command-line options:
//!
//!  1. short options with a single letter name (`-f`)
//!  2. long options with more than one letter as their name (`--file`).
//!
//! Both option types may accept an optional value or they may require a
//! value. Values are given after the option. See the section **Parsing
//! Rules** below for more information.
//!
//! Programming examples are in the **Examples** section below and in
//! the source code repository's "examples" directory.
//!
//! # Parsing Rules
//!
//! By default, all options are expected to come first in the command
//! line. Other arguments (non-options) come after options. Therefore
//! the first argument that does not look like an option stops option
//! parsing and the rest of the command line is parsed as non-options.
//! This default can be changed, so that options and non-options can be
//! mixed in their order in the command line. See [`OptSpecs::flag`]
//! method for more information.
//!
//! In command line the "pseudo option" `--` (two dashes) always stops
//! the option parser. Then the rest of the command line is parsed as
//! regular arguments (non-options).
//!
//! ## Short Options
//!
//! Short options in the command line start with the `-` character which
//! is followed by option's name character (`-c`), usually a letter.
//!
//! If option requires a value the value must be entered either directly
//! after the option character (`-cVALUE`) or as the next command-line
//! argument (`-c VALUE`). In the latter case anything that follows `-c`
//! will be parsed as option's value.
//!
//! If option accepts an optional value the value must always be entered
//! directly after the option character (`-cVALUE`). Otherwise there is
//! no value for this option.
//!
//! Several short options can be entered together after one `-`
//! character (`-abc`) but then only the last option in the series may
//! have required or optional value.
//!
//! ## Long Options
//!
//! Long options start with `--` characters and the option name comes
//! directly after it (`--foo`). The name must be at least two
//! characters long.
//!
//! If option requires a value the value must be entered either directly
//! after the option name and `=` character (`--foo=VALUE`) or as the
//! next command-line argument (`--foo VALUE`). In the latter case
//! anything that follows `--foo` will be parsed as option's value.
//!
//! If option accepts an optional value the value must always be entered
//! directly after the option name and `=` character (`--foo=VALUE`).
//! Otherwise (like in `--foo`) there is no value for this option.
//!
//! Option `--foo=` is valid format when the option requires a value or
//! accepts an optional value. It means that the value is empty string.
//! It is not valid format when the option does not accept a value.
//!
//! # Examples
//!
//! Following examples will guide through a typical usa of this library
//! crate and command-line parsing.
//!
//! ## Prepare
//!
//! First we bring some important paths into the scope of our program.
//!
//! ```
//! use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! ```
//!
//! Then we define which command-line options are valid for the program.
//! We do this by creating an instance of [`OptSpecs`] struct by calling
//! function [`OptSpecs::new`]. Then we modify the struct instance with
//! [`option`](OptSpecs::option) and [`flag`](OptSpecs::flag) methods.
//!
//! ```
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! let specs = OptSpecs::new()
//!     .option("help", "h", OptValueType::None) // Arguments: (id, name, value_type)
//!     .option("help", "help", OptValueType::None)
//!     .option("file", "f", OptValueType::Required)
//!     .option("file", "file", OptValueType::Required)
//!     .option("verbose", "v", OptValueType::Optional)
//!     .option("verbose", "verbose", OptValueType::Optional)
//!     .flag(OptFlags::OptionsEverywhere);
//! ```
//!
//! The [`option`](OptSpecs::option) methods above add a single option
//! information to the option specification. Method's arguments are:
//!
//!  1. `id`: Programmer's identifier string for the option. The same
//!     identifier is used later to check if this particular option was
//!     present in the command line.
//!
//!  2. `name`: Option's name string in the command line, without
//!     prefix. A single-character name (like `h`) defines a short
//!     option which is entered like `-h` in the command line. Longer
//!     name defines a long option which is entered like `--help` in the
//!     command line.
//!
//!  3. `value_type`: Whether or not this option accepts a value and is
//!     the value optional or required. The argument is a variant of
//!     enum [`OptValueType`].
//!
//! The [`flag`](OptSpecs::flag) method above adds a configuration flag
//! for the command-line parser. It is a variant of enum [`OptFlags`].
//! This variant [`OptionsEverywhere`](OptFlags::OptionsEverywhere)
//! changes the command-line parser to accept options and other
//! arguments in mixed order in the command line. That is, options can
//! come after non-option arguments.
//!
//! For better explanation see the documentation of [`OptSpecs`] struct
//! and its methods [`option`](OptSpecs::option) and
//! [`flag`](OptSpecs::flag).
//!
//! ## Parse the Command Line
//!
//! We are ready to parse program's command-line arguments. We do this
//! with [`OptSpecs::getopt`] method. Arguments we get from
//! [`std::env::args`] function which returns an iterator.
//!
//! ```
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! let mut args = std::env::args(); // Get arguments iterator from operating system.
//! args.next(); // Consume the first item which is this program's file path.
//! let parsed = specs.getopt(args); // Getopt! Use the "specs" variable defined above.
//! ```
//!
//! If you want to try [`getopt`](OptSpecs::getopt) method without
//! program's real command-line arguments you can also run it with other
//! iterator argument or with a vector or an array as an argument. Like
//! this:
//!
//! ```
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! let parsed = specs.getopt(["--file=123", "-f456", "foo", "-av", "bar"]);
//! ```
//!
//! ## Examine the Parsed Output
//!
//! The command line is now parsed and the variable `parsed` (see above)
//! owns an [`Args`] struct which represents the parsed output in
//! organized form. It is a public struct and it can be examined
//! manually. There are some methods for convenience, though, and some
//! of them are shown in the following examples.
//!
//! At this stage it is useful to see the returned [`Args`] struct. One
//! of its fields may contain some [`Opt`] structs too if the parser
//! found valid command-line options. Let's print it:
//!
//! ```
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! # let parsed = specs.getopt(["--file=123", "-f456", "foo", "-av", "bar"]);
//! eprintln!("{:#?}", parsed);
//! ```
//!
//! That could print something like this:
//!
//! ```text
//! Args {
//!     options: [
//!         Opt {
//!             id: "file",
//!             name: "file",
//!             value_required: true,
//!             value: Some(
//!                 "123",
//!             ),
//!         },
//!         Opt {
//!             id: "file",
//!             name: "f",
//!             value_required: true,
//!             value: Some(
//!                 "456",
//!             ),
//!         },
//!         Opt {
//!             id: "verbose",
//!             name: "v",
//!             value_required: false,
//!             value: None,
//!         },
//!     ],
//!     other: [
//!         "foo",
//!         "bar",
//!     ],
//!     unknown: [
//!         "a",
//!     ],
//! }
//! ```
//!
//! The returned [`Args`] struct above can be examined manually but the
//! struct has some methods to make things convenient.
//!
//! ### Unknown Options
//!
//! We probably want to tell program's user if there were unknown
//! options. An error message to [`std::io::stderr`] stream is usually
//! enough. No need to panic.
//!
//! ```
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! # let parsed = specs.getopt(["--file=123", "-f456", "foo", "-av", "bar"]);
//! for u in &parsed.unknown {
//!     eprintln!("Unknown option: {}", u);
//! }
//! ```
//!
//! ### Required Value Missing
//!
//! More serious error is a missing value to an option which requires a
//! value (like `file` option in our example, see above). That can be a
//! good reason to exit the program.
//!
//! ```no_run
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! # let parsed = specs.getopt(["--file=123", "-f456", "foo", "-av", "bar"]);
//! for o in &parsed.required_value_missing() {
//!     eprintln!("Value is required for option '{}'.", o.name);
//!     std::process::exit(1);
//! }
//! ```
//!
//! ### Print Help Message
//!
//! Command-line programs always have `-h` or `--help` option for
//! printing a friendly help message. The following example shows how to
//! detect that option.
//!
//! ```no_run
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! # let parsed = specs.getopt(["--file=123", "-f456", "foo", "-av", "bar"]);
//! if let Some(_) = parsed.options_first("help") {
//!     println!("Print friendly help about program's usage.");
//!     std::process::exit(2);
//! }
//! ```
//!
//! The `"help"` string in the first line above is the identifier string
//! (`id`) for the option. It was defined with [`OptSpecs::option`]
//! method in the example code earlier. Identifier strings are used to
//! find if a specific option was given in the command line.
//!
//! ### Collect Values and Other Arguments
//!
//! The rest depends very much on individual program's needs. Probably
//! often we would collect what values were given to options. In our
//! example program there are `-f` and `--file` options that require a
//! value. We could collect all those values next.
//!
//! ```
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! # let parsed = specs.getopt(["--file=123", "-f456", "foo", "-av", "bar"]);
//! for f in &parsed.options_value_all("file") {
//!     println!("File name: {:?}", f);
//! }
//! ```
//!
//! Notice if `-v` or `--verbose` was given, even without a value. Then
//! collect all (optional) values for the option.
//!
//! ```
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! # let parsed = specs.getopt(["--file=123", "-f456", "foo", "-av", "bar"]);
//! if let Some(_) = parsed.options_first("verbose") {
//!     println!("Option 'verbose' was given.");
//!
//!     for v in &parsed.options_value_all("verbose") {
//!         println!("Verbose level: {:?}", v);
//!     }
//! }
//! ```
//!
//! Finally, our example program will handle all other arguments, that
//! is, non-option arguments.
//!
//! ```
//! # use just_getopt::{OptFlags, OptSpecs, OptValueType};
//! # let specs = OptSpecs::new();
//! # let parsed = specs.getopt(["--file=123", "-f456", "foo", "-av", "bar"]);
//! for o in &parsed.other {
//!     println!("Other argument: {:?}", o);
//! }
//! ```
//!
//! # More Help
//!
//! A complete working example code -- very similar to previous examples
//! -- is in the source code repository's "examples" directory. It can
//! be run with command `cargo run --example basic -- your arguments`.
//! Try it with different command-line arguments.
//!
//! Further reading:
//!
//!   - [`OptSpecs`] struct and its methods.
//!   - [`Args`] struct and its methods.

mod parser;

#[cfg(test)]
mod tests;

/// Specification for program's valid command-line options.
///
/// An instance of this struct is needed before command-line options can
/// be parsed. Instances are created with function [`OptSpecs::new`] and
/// they are modified with methods [`option`](OptSpecs::option) and
/// [`flag`](OptSpecs::flag).
///
/// The struct instance is used when parsing the command line given by
/// program's user. The parser methods is [`getopt`](OptSpecs::getopt).

#[derive(Debug, PartialEq)]
pub struct OptSpecs {
    options: Vec<OptSpec>,
    flags: Vec<OptFlags>,
}

#[derive(Debug, PartialEq)]
struct OptSpec {
    id: String,
    name: String,
    value_type: OptValueType,
}

/// Option's value type.
///
/// See [`OptSpecs::option`] method for more information.

#[derive(Debug, PartialEq)]
pub enum OptValueType {
    None,
    Optional,
    Required,
}

/// Flags for changing command-line parser's behavior.
///
/// See [`OptSpecs::flag`] method for more information.

#[derive(Debug, PartialEq)]
pub enum OptFlags {
    OptionsEverywhere,
    PrefixMatchLongOptions,
}

impl OptSpecs {
    /// Create and return a new instance of [`OptSpecs`] struct.
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            flags: Vec::new(),
        }
    }

    /// Add an option specification for [`OptSpecs`].
    ///
    /// The method requires three arguments:
    ///
    ///  1. `id`: Programmer's identifier string for the option. Later,
    ///     after parsing the command line, the identifier is used to
    ///     match if this particular option was present in the
    ///     command-line.
    ///
    ///     Several options may have the same identifier string. This
    ///     makes sense when different option names in the command line
    ///     represent the same meaning, like `-h` and `--help` for
    ///     printing program's help message.
    ///
    ///  2. `name`: Option's name string in the command line (without
    ///     prefix). If the string is a single character (like `h`) it
    ///     defines a short option which is entered as `-h` in the
    ///     command line. If there are more than one character in the
    ///     string it defines a long option name (like `help`) which is
    ///     entered as `--help` in the command line.
    ///
    ///     All options must have a unique `name` string. This method
    ///     will panic if the same `name` is added twice. The method
    ///     will also panic if the `name` string contains illegal
    ///     characters. Space characters are not accepted. A short
    ///     option name can't be `-` and long option names can't have
    ///     any `=` characters nor `-` as their first character.
    ///
    ///  3. `value_type`: A variant of enum [`OptValueType`] which
    ///     defines if this option accepts a value. If not, use
    ///     [`OptValueType::None`] as method's argument. If an optional
    ///     value is accepted, use [`OptValueType::Optional`]. If the
    ///     option requires a value, use [`OptValueType::Required`].
    ///
    /// Method returns the same [`OptSpecs`] struct instance which was
    /// modified.

    pub fn option(mut self: Self, id: &str, name: &str, value_type: OptValueType) -> Self {
        assert!(
            id.chars().count() > 0,
            "Option's \"id\" must be at least 1 character long."
        );

        let name_count = name.chars().count();

        if name_count == 1 {
            assert!(
                parser::is_valid_short_option_name(name),
                "Not a valid short option name."
            );
        } else if name_count >= 2 {
            assert!(
                parser::is_valid_long_option_name(name),
                "Not a valid long option name."
            );
        } else {
            panic!("Option's \"name\" must be at least 1 character long.");
        }

        for e in &self.options {
            assert!(
                e.name != name,
                "No duplicates allowed for option's \"name\"."
            );
        }

        self.options.push(OptSpec {
            id: id.to_string(),
            name: name.to_string(),
            value_type: value_type,
        });
        self
    }

    /// Add a flag that changes parser's behavior.
    ///
    /// Method's only argument `flag` is a variant of enum [`OptFlags`].
    /// Their names and meanings are:
    ///
    ///   - [`OptFlags::OptionsEverywhere`]: Accept command-line options
    ///     and other arguments in mixed order in the command line. That
    ///     is, options can come after non-option arguments.
    ///
    ///     This is not the default behavior. By default the first
    ///     non-option argument in the command line stops option parsing
    ///     and the rest of the command line is parsed as non-options
    ///     (other arguments), even if they look like options.
    ///
    ///   - [`OptFlags::PrefixMatchLongOptions`]: With this flag long
    ///     options don't need to be written in full in the command
    ///     line. They can be shortened as long as there are enough
    ///     characters to find a unique prefix match. If there are more
    ///     than one match the option given in the command line is
    ///     classified as unknown.
    ///
    /// Method returns the same [`OptSpecs`] struct instance which was
    /// modified.

    pub fn flag(mut self: Self, flag: OptFlags) -> Self {
        self.flags.push(flag);
        self
    }

    fn is_flag(self: &Self, flag: OptFlags) -> bool {
        self.flags.contains(&flag)
    }

    /// Getopt-parse an iterable item as command line arguments.
    ///
    /// This method's argument `args` is of any type that implements
    /// trait [`IntoIterator`] and that has items of type that
    /// implements trait [`ToString`]. For example, argument `args` can
    /// be a vector or an iterator such as command-line arguments
    /// returned by [`std::env::args`].
    ///
    /// The return value is an [`Args`] struct which represents the
    /// command-line information in organized form.

    pub fn getopt<I, S>(self: &Self, args: I) -> Args
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        parser::parse(&self, args.into_iter().map(|i| i.to_string()))
    }

    fn get_short_option_match(self: &Self, name: &str) -> Option<&OptSpec> {
        if name.chars().count() != 1 {
            return None;
        }
        for e in &self.options {
            if e.name == name {
                return Some(e);
            }
        }
        None
    }

    fn get_long_option_match(self: &Self, name: &str) -> Option<&OptSpec> {
        if name.chars().count() < 2 {
            return None;
        }
        for e in &self.options {
            if e.name == name {
                return Some(e);
            }
        }
        None
    }

    fn get_long_option_prefix_matches(self: &Self, name: &str) -> Option<Vec<&OptSpec>> {
        if name.chars().count() < 2 {
            return None;
        }

        let mut v = Vec::new();
        for e in &self.options {
            if e.name.starts_with(name) {
                v.push(e);
            }
        }

        if v.len() > 0 {
            Some(v)
        } else {
            None
        }
    }
}

/// Parsed command line in organized form.
///
/// Instances of this struct are usually created with
/// [`OptSpecs::getopt`] method and an instance represents the parsed
/// output in organized form. See each field's documentation for more
/// information.
///
/// Programmers can use the parsed output ([`Args`] struct) any way they
/// like. There are some methods for convenience.

#[derive(Debug, PartialEq)]
pub struct Args {
    /// A vector of valid command-line options.
    ///
    /// Elements of this vector are [`Opt`] structs which each
    /// represents a single command-line option. Elements are in the
    /// same order as given (by program's user) in the command line. The
    /// vector is empty if the parser didn't find any valid command-line
    /// options.
    pub options: Vec<Opt>,

    /// A vector of other arguments (non-options).
    ///
    /// Each element of the vector is a single non-option argument
    /// string in the same order as given (by program's user) in the
    /// command line. The vector is empty if the parser didn't find any
    /// non-option arguments.
    pub other: Vec<String>,

    /// Unknown options.
    ///
    /// Command-line arguments that look like options but were not part
    /// of [`OptSpecs`] specification are classified as unknown. They
    /// are listed in this vector. Possible duplicate unknown options
    /// given in command line have been filtered.
    ///
    /// Each element is the name string for the option (without `-` or
    /// `--` prefix). For unknown short options the element is a
    /// single-character string. For unknown long options the string has
    /// more than one character. The whole vector is empty if there were
    /// no unknown options.
    ///
    /// If a long option does not accept a value (that is, its value
    /// type is [`OptValueType::None`]) but user gives it a value with
    /// equal sign notation (`--foo=`), that option is classified as
    /// unknown and it will be in this field's vector with name `foo=`.
    pub unknown: Vec<String>,
}

impl Args {
    fn new() -> Self {
        Args {
            options: Vec::new(),
            other: Vec::new(),
            unknown: Vec::new(),
        }
    }

    /// Find options with missing required value.
    ///
    /// This method finds all (otherwise valid) options which require a
    /// value but the value is missing. That is, [`OptSpecs`] struct
    /// specification defined that an option requires a value but
    /// program's user didn't give one in the command line. Such thing
    /// can happen if an option like `--file` is the last argument in
    /// the command line and that option requires a value. Empty string
    /// `""` is not classified as missing value because it can be valid
    /// user input in many situations.
    ///
    /// This method returns a vector (possibly empty) and each element
    /// is a reference to an [`Opt`] struct in the original
    /// [`Args::options`] field contents.

    pub fn required_value_missing(self: &Self) -> Vec<&Opt> {
        let mut vec = Vec::new();
        for opt in &self.options {
            if opt.value_required && opt.value.is_none() {
                vec.push(opt);
            }
        }
        vec
    }

    /// Find all options with the given `id`.
    ///
    /// Find all options which have the identifier `id`. (Option
    /// identifiers have been defined in [`OptSpecs`] struct before
    /// parsing.) The return value is a vector (possibly empty, if no
    /// matches) and each element is a reference to [`Opt`] struct in
    /// the original [`Args`] struct. Elements in the vector are in the
    /// same order as in the parsed command line.

    pub fn options_all(self: &Self, id: &str) -> Vec<&Opt> {
        let mut vec = Vec::new();
        for opt in &self.options {
            if opt.id == id {
                vec.push(opt);
            }
        }
        vec
    }

    /// Find the first option with the given `id`.
    ///
    /// Find and return the first match for option `id` in command-line
    /// arguments' order. (Options' identifiers have been defined in
    /// [`OptSpecs`] struct before parsing.)
    ///
    /// The return value is a variant of enum [`Option`]. Their
    /// meanings:
    ///
    ///   - `None`: No options found with the given `id`.
    ///
    ///   - `Some(&Opt)`: An option was found with the given `id` and a
    ///     reference to its [`Opt`] struct in the original [`Args`]
    ///     struct is provided.

    pub fn options_first(self: &Self, id: &str) -> Option<&Opt> {
        for opt in &self.options {
            if opt.id == id {
                return Some(opt);
            }
        }
        None
    }

    /// Find the last option with the given `id`.
    ///
    /// This is similar to [`options_first`](Args::options_first) method
    /// but this returns the last match in command-line arguments'
    /// order.

    pub fn options_last(self: &Self, id: &str) -> Option<&Opt> {
        for opt in self.options.iter().rev() {
            if opt.id == id {
                return Some(opt);
            }
        }
        None
    }

    /// Find and return all values for options with the given `id`.
    ///
    /// Find all options which match the identifier `id` and which also
    /// have a value assigned. (Options' identifiers have been defined
    /// in [`OptSpecs`] struct before parsing.) Collect options' values
    /// into a new vector in the same order as they were given in the
    /// command line. Vector's elements are references to the value
    /// strings in the original [`Args`] struct. The returned vector is
    /// empty if there were no matches.

    pub fn options_value_all(self: &Self, id: &str) -> Vec<&String> {
        let mut vec = Vec::new();
        let opt_vec = self.options_all(id);
        for opt in opt_vec {
            if let Some(s) = &opt.value {
                vec.push(s);
            }
        }
        vec
    }

    /// Find the first option with a value for given option `id`.
    ///
    /// Find the first option which match the identifier `id` and which
    /// has a value assigned. (Options' identifiers have been defined in
    /// [`OptSpecs`] struct before parsing.) Method's return value is a
    /// variant of enum [`Option`] which are:
    ///
    ///   - `None`: No options found with the given `id`, an option
    ///     which also has a value assigned. There could be options for
    ///     the same `id` but they don't have a value.
    ///
    ///   - `Some(&String)`: An option was found with the given `id` and
    ///     the option has a value assigned. A reference to the string
    ///     value in the original [`Args`] struct is provided.

    pub fn options_value_first(self: &Self, id: &str) -> Option<&String> {
        let all = self.options_value_all(id);
        if all.len() > 0 {
            Some(all[0])
        } else {
            None
        }
    }

    /// Find the last option with a value for given option `id`.
    ///
    /// This is similar to
    /// [`options_value_first`](Args::options_value_first) method but
    /// this method finds and returns the last option's value.
    ///
    /// Note: Program's user may give the same option several times in
    /// the command line. If the option accepts a value it may be
    /// suitable to consider only the last value relevant. (Or the
    /// first, or maybe print an error message for providing several,
    /// possibly conflicting, values.)

    pub fn options_value_last(self: &Self, id: &str) -> Option<&String> {
        let all = self.options_value_all(id);
        let len = all.len();
        if len > 0 {
            Some(all[len - 1])
        } else {
            None
        }
    }
}

/// Structured option information.
///
/// This [`Opt`] struct represents organized information about single
/// command-line option. Instances of this struct are usually created by
/// [`OptSpecs::getopt`] method which returns an [`Args`] struct which
/// have these [`Opt`] structs inside.
///
/// A programmer may need these when examining parsed command-line
/// options. See the documentation of individual fields for more
/// information. Also see [`Args`] struct and its methods.

#[derive(Debug, PartialEq)]
pub struct Opt {
    /// Identifier for the option.
    ///
    /// Identifiers are defined with [`OptSpecs::option`] method before
    /// parsing command-line arguments. After [`OptSpecs::getopt`]
    /// parsing the same identifier is copied here and it confirms that
    /// the option was indeed given in the command line.
    pub id: String,

    /// Option's name in the parsed command line.
    ///
    /// Option's name that was used in the command line. For short
    /// options this is a single-character string. For long options the
    /// name has more than one characters.
    pub name: String,

    /// The option requires a value.
    ///
    /// `true` means that the option was defined with value type
    /// [`OptValueType::Required`]. See [`OptSpecs::flag`] method for
    /// more information. This field does not guarantee that there
    /// actually was a value for the option in the command line.
    pub value_required: bool,

    /// Option's value.
    ///
    /// The value is a variant of enum [`Option`]. Value `None` means
    /// that there is no value for the option. Value `Some(String)`
    /// provides a value.
    pub value: Option<String>,
}
