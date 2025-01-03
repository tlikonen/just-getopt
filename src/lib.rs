mod parser;

#[cfg(test)]
mod tests;

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

#[derive(Debug, PartialEq)]
pub enum OptValueType {
    None,
    Optional,
    Required,
}

#[derive(Debug, PartialEq)]
pub enum OptFlags {
    OptionsEverywhere,
    PrefixMatchLongOptions,
}

impl OptSpecs {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            flags: Vec::new(),
        }
    }

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
