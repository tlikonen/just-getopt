//! # Introduction
//!
//! This library crate implements a Posix `getopt`-like command-line
//! option parser with simple programming interface. More specifically
//! the parser is like `getopt`’s GNU extension called `getopt_long`
//! which is familiar command-line option format for users of
//! Linux-based operating systems.
//!
//! The name is `just_getopt` because this is *just a getopt parser* and
//! (almost) nothing more. The intent is to provide just the parsed
//! output and methods for examining the output. There will not be
//! anything for interpreting the output or for printing messages to
//! program’s user. The responsibility of interpretation is left to your
//! program.
//!
//! In getopt logic there are two types of command-line options:
//!
//!  1. short options with a single letter name (`-f`)
//!  2. long options with more than one letter as their name (`--file`).
//!
//! Both option types may accept an optional value or they may require a
//! value. Values are given after the option. See the section [Parsing
//! rules](#parsing-rules) below for more information.
//!
//! Programming examples are in the [Examples](#examples) section below.
//!
//! # Parsing rules
//!
//! By default, all options are expected to come first in the command
//! line. Other arguments (non-options) come after options. Therefore
//! the first argument that does not look like an option stops option
//! parsing and the rest of the command line is parsed as non-options.
//! This default can be changed, so that options and non-options can be
//! mixed in their order in the command line. See `OptSpecs` struct’s
//! `flag()` method for more information.
//!
//! In command line the “pseudo option” `--` (two dashes) always stops
//! the option parser. Then the rest of the command line is parsed as
//! regular arguments (non-options).
//!
//! ## Short options
//!
//! Short options in the command line start with the `-` character which
//! is followed by option’s name character (`-c`), usually a letter.
//!
//! If option requires a value the value must be entered either directly
//! after the option character (`-cVALUE`) or as the next command-line
//! argument (`-c VALUE`). In the latter case anything that follows `-c`
//! will be parsed as option’s value.
//!
//! If option accepts an optional value the value must always be entered
//! directly after the option character (`-cVALUE`). Otherwise there is
//! no value for this option.
//!
//! Several short options can be entered together after one `-`
//! character (`-abc`) but then only the last option in the series may
//! have required or optional value.
//!
//! ## Long options
//!
//! Long options start with `--` characters and the option name comes
//! directly after it (`--foo`). The name must be at least two
//! characters long.
//!
//! If option requires a value the value must be entered either directly
//! after the option name and `=` character (`--foo=VALUE`) or as the
//! next command-line argument (`--foo VALUE`). In the latter case
//! anything that follows `--foo` will be parsed as option’s value.
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
//! (not yet)

mod parser;

#[cfg(test)]
mod tests;

/// Specification for program’s valid command-line options.
///
/// An instance of this struct is needed before command-line options can
/// be parsed. Instances are created with function `OptSpecs::new()` and
/// they are modified with methods `option()` and `flag()`.
///
/// The struct instance is used when parsing the command line given by
/// program’s user. Parser methods are `getopt()` and `getopt_vec()`.

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

/// Parsed command line in organized form.
///
/// Instances of this struct are usually created with `OptSpecs`
/// struct’s `getopt()` method and an instance represents the parsed
/// output in organized form. See each field’s documentation for more
/// information.
///
/// A programmer can use the parsed output (`Args` struct) any way they
/// like. There are some methods for convenience.

#[derive(Debug, PartialEq)]
pub struct Args {
    /// A vector of valid command-line options.
    ///
    /// Elements of this vector are `Opt` structs which each represents
    /// a single command-line option. Elements are in the same order as
    /// given (by program’s user) in the command line. The vector is
    /// empty if the parser didn’t find any valid command-line options.
    pub options: Vec<Opt>,

    /// A vector of other arguments (non-options).
    ///
    /// Each element of the vector is a single non-option argument
    /// string in the same order as given (by program’s user) in the
    /// command line. The vector is empty if the parser didn’t find any
    /// non-option arguments.
    pub other: Vec<String>,

    /// Unknown options.
    ///
    /// Command-line arguments that look like options but were not part
    /// of `OptSpecs` specification are classified as unknown. They are
    /// listed in this vector. Each element is the name string for the
    /// option (without `-` or `--` prefix). For unknown short options
    /// the element is a single-character string. For unknown long
    /// options the string has more than one character. The whole vector
    /// is empty if there were no unknown options.
    pub unknown: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Opt {
    pub id: String,
    pub name: String,
    pub value_required: bool,
    pub value: Option<String>,
}

/// Option’s value type.
///
/// See `OptSpecs` struct’s `option()` method for more information.

#[derive(Debug, PartialEq)]
pub enum OptValueType {
    None,
    Optional,
    Required,
}

/// Flags for changing command-line parser’s behavior.
///
/// See `OptSpecs` struct’s `flag()` method for more information.

#[derive(Debug, PartialEq)]
pub enum OptFlags {
    OptionsEverywhere,
    PrefixMatchLongOptions,
}

impl OptSpecs {
    /// Create and return a new instance of `OptSpecs` struct.
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            flags: Vec::new(),
        }
    }

    /// Add an option specification for `OptSpecs`.
    ///
    /// The method requires three arguments:
    ///
    ///  1. `id`: Programmer’s identifier string for the option. Later,
    ///     after parsing, the identifier is used to match if this
    ///     particular option was present in the command-line.
    ///
    ///     Several options may have the same identifier string. This
    ///     makes sense when different option names in the command line
    ///     represent the same meaning, like `-h` and `--help` for
    ///     printing program’s help message.
    ///
    ///  2. `name`: Option’s name string in the command line (without
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
    ///     option name can’t be `-` and long option names can’t have
    ///     any `=` characters nor `-` as their first character.
    ///
    ///  3. `value_type`: A variant of enum `OptValueType` which defines
    ///     if this option accepts a value. If not, use
    ///     `OptValueType::None` as method’s argument. If an optional
    ///     value is accepted, use `OptValueType::Optional`. If the
    ///     option requires a value, use `OptValueType::Required`.
    ///
    /// Method returns the same `OptSpecs` struct instance which was
    /// modified.

    pub fn option(mut self: Self, id: &str, name: &str, value_type: OptValueType) -> Self {
        assert!(
            id.len() > 0,
            "Option's \"id\" must be at least 1 character long."
        );

        if name.len() == 1 {
            assert!(
                parser::is_valid_short_option_name(name),
                "Not a valid short option name."
            );
        } else if name.len() >= 2 {
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

    /// Add a flag that changes parser’s behavior.
    ///
    /// Method’s only argument `flag` is a variant of enum `OptFlags`.
    /// Their names and meanings are:
    ///
    ///   - `OptFlags::OptionsEverywhere`: Accept command-line options
    ///     and other arguments in mixed order in the command line. That
    ///     is, options can come after non-option arguments.
    ///
    ///     This is not the default behavior. By default the first
    ///     non-option argument in the command line stops option parsing
    ///     and the rest of the command line is parsed as non-options
    ///     (other arguments), even if they look like options.
    ///
    ///   - `OptFlags::PrefixMatchLongOptions`: With this flag long
    ///      options don’t need to be written in full in the command
    ///      line. They can be shortened as long as there are enough
    ///      characters to find a unique prefix match. If there are more
    ///      than one match the option given in the command line is
    ///      classified as unknown.
    ///
    /// Method returns the same `OptSpecs` struct instance which was
    /// modified.

    pub fn flag(mut self: Self, flag: OptFlags) -> Self {
        self.flags.push(flag);
        self
    }

    fn is_flag(self: &Self, flag: OptFlags) -> bool {
        self.flags.contains(&flag)
    }

    /// Parse program’s command line.
    ///
    /// This method parses current program’s command line arguments
    /// (`std::env::args()`). It returns an instance of `Args` struct
    /// which contains the command line information in organized form.
    /// See the documentation of `Args` struct for more information.

    pub fn getopt(self: &Self) -> Args {
        let mut iter = std::env::args();
        iter.next();
        let vec = iter.collect();
        parser::parse(&self, &vec)
    }

    /// Parse a vector as command-line.
    ///
    /// This method is similar to `getopt()` except that this method
    /// parses the argument `args` which is a vector of strings. Each
    /// element in the vector is an argument in command line.

    pub fn getopt_vec(self: &Self, args: &Vec<String>) -> Args {
        parser::parse(&self, args)
    }

    fn get_short_option_match(self: &Self, name: &str) -> Option<&OptSpec> {
        if name.len() != 1 {
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
        if name.len() < 2 {
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
        if name.len() < 2 {
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
    /// value but the value is missing. That is, `OptSpecs` struct
    /// specification defined that an option requires a value but
    /// program’s user didn’t give one in the command line. Such thing
    /// can happen if an option like `--file` is the last argument in
    /// the command line and that option requires a value. Empty string
    /// `""` is not classified as missing value because it can be valid
    /// user input in many situations.
    ///
    /// This method returns a vector (possibly empty) and each element
    /// is a reference to an `Opt` struct in the original `Args`
    /// struct’s `options` field contents.

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
    /// identifiers have been defined in `OptSpecs` structs before
    /// parsing.) The return value is a vector (possibly empty, if no
    /// matches) and each element is a reference to `Opt` struct in the
    /// original `Args` struct.

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
    /// arguments’ order. (Options’ identifiers have been defined in
    /// `OptSpecs` struct before parsing.)
    ///
    /// The return value is a variant of enum `Option`. Their meanings:
    ///
    ///   - `None`: No options found with the given `id`.
    ///
    ///   - `Some(&Opt)`: An option was found with the given `id` and a
    ///     reference to its `Opt` struct in the original `Args` struct
    ///     is provided.

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
    /// This is similar to `options_first()` method but this returns the
    /// last match in command-line arguments’ order.

    pub fn options_last(self: &Self, id: &str) -> Option<&Opt> {
        let len = self.options.len();
        let mut pos;
        for i in 0..len {
            pos = len - i - 1;
            if self.options[pos].id == id {
                return Some(&self.options[pos]);
            }
        }
        None
    }

    /// Find and return all values for options with the given `id`.
    ///
    /// Find all options which match the identifier `id` and which have
    /// a value assigned. (Options’ identifiers have been defined in
    /// `OptSpecs` struct before parsing.) Collect options’ values into
    /// a new vector. Vector’s elements are references to the value
    /// strings in the original `Args` struct. The returned vector is
    /// empty if there were no matches.

    pub fn options_value_all(self: &Self, id: &str) -> Vec<&String> {
        let mut vec = Vec::new();
        let opt_vec = self.options_all(id);
        for opt in opt_vec {
            match &opt.value {
                None => (),
                Some(s) => vec.push(s),
            }
        }
        vec
    }

    /// Find the first option with a value for given option `id`.
    ///
    /// Find the first option which match the identifier `id` and which
    /// has a value assigned. (Options’ identifiers have been defined in
    /// `OptSpecs` struct before parsing.) Method’s return value is a
    /// variant of enum `Option` which are:
    ///
    ///   - `None`: No options found with the given `id`, options which
    ///     also have a value assigned. There could be options for the
    ///     same `id` but they don’t have a value.
    ///
    ///   - `Some(&String)`: An option was found with the given `id` and
    ///     the option has a value assigned. A reference to the string
    ///     value in the original `Args` struct is provided.

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
    /// This is similar to `options_value_first()` method but this
    /// method find and returns the last option’s value.
    ///
    /// Program’s user may give the same option several times in the
    /// command line. If the option accepts a value it may be suitable
    /// to consider only the last value relevant. (Or the first, or
    /// maybe print an error message for providing several values.)

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
