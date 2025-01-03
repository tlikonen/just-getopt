//! # Introduction
//!
//! This crate implements a Posix `getopt`-like command-line option
//! parser with friendly Rust-like programming interface. It’s *just*
//! that, hence the name `just_getopt`. The intent is to not add fancy
//! features or automatic magic.
//!
//! More specifically the parser is like `getopt`’s GNU extension called
//! `getopt_long` which is familiar format for users of Linux-based
//! operating systems.
//!
//! There are two types of command-line options:
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
//! In command line the “pseudo option” `--` (two dashes) stops parsing
//! options. The rest of the command line is parsed as regular arguments
//! (that is, non-options).
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
/// An instance is this struct is needed before command-line options can
/// be parsed. Instances are created with function `OptSpecs::new()` and
/// they are modified with methods `option()` and `flag()`.
///
/// The struct instance is used to parse command line given by program’s
/// user. The parser methods are `getopt()` and `getopt_vec()`.

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

#[derive(Debug, PartialEq)]
pub struct Args {
    pub options: Vec<Opt>,
    pub other: Vec<String>,
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

/// Option flags which change command-line parser’s behaviour.
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
    ///     characters. Space characters are not accepted. Short option
    ///     name can’t be `-` and long option names can’t have any `=`
    ///     characters nor `-` as their first character.
    ///
    ///  3. `value_type`: A variant of enum `OptValueType` which defines
    ///     if this option accepts a value. If not, use
    ///     `OptValueType::None` as method’s argument. If optional value
    ///     is accepted, use `OptValueType::Optional`. If the option
    ///     requires a value, use `OptValueType::Required`.
    ///
    /// Method’s return value is the same `OptSpecs` struct instance
    /// which was modified.

    pub fn option(mut self: Self, id: &str, name: &str, value_type: OptValueType) -> Self {
        assert!(id.len() > 0,
                "Option's \"id\" must be at least 1 character long.");

        if name.len() == 1 {
            assert!(parser::is_valid_short_option_name(name),
                    "Not a valid short option name.");
        } else if name.len() >= 2 {
            assert!(parser::is_valid_long_option_name(name),
                    "Not a valid long option name.");
        } else {
            panic!("Option's \"name\" must be at least 1 character long.");
        }

        for e in &self.options {
            assert!(e.name != name, "No duplicates allowed for option's \"name\".");
        }

        self.options.push(OptSpec {
            id: id.to_string(),
            name: name.to_string(),
            value_type: value_type,
        });
        self
    }

    /// Add a flag that changes parser’s behaviour.
    ///
    /// Method’s only argument `flag` is a variant of enum `OptFlags`.
    /// Their names and meanings are:
    ///
    ///   - `OptFlags::OptionsEverywhere`: Accept command-line options
    ///     and other arguments in mixed order in the command line. That
    ///     is, options can come after non-option arguments.
    ///
    ///     This is not the default behaviour. By default the first
    ///     non-option argument in the command line stops option parsing
    ///     and the rest of the command line is parsed as non-options
    ///     (“other arguments”), even if they look like options.
    ///
    ///   - `OptFlags::PrefixMatchLongOptions`: Long options don’t need
    ///      to be written in full in the command line. They can be
    ///      shortened as long as there are enough characters to find a
    ///      unique prefix match. If there are more than one match the
    ///      option is classified as unknown.
    ///
    /// Method’s return value is the same `OptSpecs` struct instance
    /// which was modified.

    pub fn flag(mut self: Self, flag: OptFlags) -> Self {
        self.flags.push(flag);
        self
    }

    fn is_flag(self: &Self, flag: OptFlags) -> bool {
        self.flags.contains(&flag)
    }

    pub fn getopt(self: &Self) -> Args {
        let mut iter = std::env::args();
        iter.next();
        let vec = iter.collect();
        parser::parse(&self, &vec)
    }

    pub fn getopt_vec(self: &Self, args: &Vec<String>) -> Args {
        parser::parse(&self, args)
    }

    fn get_short_option_match(self: &Self, name: &str) -> Option<&OptSpec> {
        if name.len() != 1 { return None; }
        for e in &self.options {
            if e.name == name { return Some(e); }
        }
        None
    }

    fn get_long_option_match(self: &Self, name: &str) -> Option<&OptSpec> {
        if name.len() < 2 { return None; }
        for e in &self.options {
            if e.name == name { return Some(e); }
        }
        None
    }

    fn get_long_option_prefix_matches(self: &Self, name: &str) -> Option<Vec<&OptSpec>> {
        if name.len() < 2 { return None; }

        let mut v = Vec::new();
        for e in &self.options {
            if e.name.starts_with(name) { v.push(e); }
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

    pub fn options_first(self: &Self, id: &str) -> Option<&Opt> {
        for opt in &self.options {
            if opt.id == id { return Some(opt); }
        }
        None
    }

    pub fn options_last(self: &Self, id: &str) -> Option<&Opt> {
        let len = self.options.len();
        let mut pos;
        for i in 0..len {
            pos = len - i - 1;
            if self.options[pos].id == id { return Some(&self.options[pos]); }
        }
        None
    }

    pub fn options_all(self: &Self, id: &str) -> Vec<&Opt> {
        let mut vec = Vec::new();
        for opt in &self.options {
            if opt.id == id { vec.push(opt); }
        }
        vec
    }

    pub fn required_value_missing(self: &Self) -> Vec<&Opt> {
        let mut vec = Vec::new();
        for opt in &self.options {
            if opt.value_required && opt.value.is_none() {
                vec.push(opt);
            }
        }
        vec
    }

    pub fn options_value_first(self: &Self, id: &str) -> Option<&String> {
        options_value_engine(&self.options_first(id))
    }

    pub fn options_value_last(self: &Self, id: &str) -> Option<&String> {
        options_value_engine(&self.options_last(id))
    }

    pub fn options_value_all(self: &Self, id: &str) -> Vec<&String> {
        let mut vec = Vec::new();
        let opt_vec = self.options_all(id);
        for opt in opt_vec {
            match &opt.value {
                None => (),
                Some(v) => vec.push(v),
            }
        }
        vec
    }
}

fn options_value_engine<'a>(option: &Option<&'a Opt>) -> Option<&'a String> {
    match option {
        None => None,
        Some(opt) => {
            match &opt.value {
                None => None,
                Some(value) => Some(value),
            }
        },
    }
}
