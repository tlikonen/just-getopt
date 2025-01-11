use crate::{Args, Opt, OptFlags, OptSpecs, OptValueType};

#[cfg(test)]
mod tests;

pub fn parse<I>(specs: &OptSpecs, mut iter: I) -> Args
where
    I: Iterator<Item = String>,
{
    let mut parsed = Args::new();

    loop {
        let opt = match iter.next() {
            None => break,
            Some(v) => v.clone(),
        };

        if is_option_terminator(&opt) {
            break;
        } else if is_long_option_prefix(&opt) {
            let name = get_long_option_name(&opt).to_string();

            if is_valid_long_option_name(&name) {
                let spec_test = if specs.is_flag(OptFlags::PrefixMatchLongOptions) {
                    match specs.get_long_option_prefix_matches(&name) {
                        None => None,
                        Some(vec) => {
                            if vec.len() == 1 {
                                Some(vec[0])
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    specs.get_long_option_match(&name)
                };

                if let Some(spec) = spec_test {
                    let value_required: bool;
                    let value: Option<String>;

                    match spec.value_type {
                        OptValueType::Required => {
                            value_required = true;
                            value = if is_long_option_equal_sign(&opt) {
                                Some(get_long_option_equal_value(&opt).to_string())
                            } else {
                                iter.next()
                            }
                        }

                        OptValueType::Optional => {
                            value_required = false;
                            value = if is_long_option_equal_sign(&opt) {
                                Some(get_long_option_equal_value(&opt).to_string())
                            } else {
                                None
                            }
                        }

                        OptValueType::None => {
                            value_required = false;
                            value = None;
                            if is_long_option_equal_sign(&opt) {
                                let n = format!("{}=", name);
                                if !parsed.unknown.contains(&n) {
                                    parsed.unknown.push(n);
                                }
                                continue;
                            }
                        }
                    }

                    parsed.options.push(Opt {
                        id: spec.id.clone(),
                        name: name,
                        value_required: value_required,
                        value: value,
                    });
                    continue;
                }
            }

            if !parsed.unknown.contains(&name) {
                parsed.unknown.push(name);
            }
            continue;
        } else if is_short_option_prefix(&opt) {
            let series = get_short_option_series(&opt);
            let mut char_iter = series.chars();

            loop {
                let name = match char_iter.next() {
                    None => break,
                    Some(v) => v.to_string(),
                };

                if is_valid_short_option_name(&name) {
                    if let Some(spec) = specs.get_short_option_match(&name) {
                        let value_required: bool;
                        let value: Option<String>;

                        match spec.value_type {
                            OptValueType::Required => {
                                value_required = true;
                                let mut chars = String::new();
                                while let Some(c) = char_iter.next() {
                                    chars.push(c);
                                }
                                value = if chars.chars().count() > 0 {
                                    Some(chars)
                                } else {
                                    iter.next()
                                }
                            }

                            OptValueType::Optional => {
                                value_required = false;
                                let mut chars = String::new();
                                while let Some(c) = char_iter.next() {
                                    chars.push(c);
                                }
                                value = if chars.chars().count() > 0 { Some(chars) } else { None }
                            }

                            OptValueType::None => {
                                value_required = false;
                                value = None;
                            }
                        }

                        parsed.options.push(Opt {
                            id: spec.id.clone(),
                            name: name,
                            value_required: value_required,
                            value: value,
                        });
                        continue;
                    }
                }

                if !parsed.unknown.contains(&name) {
                    parsed.unknown.push(name);
                }
                continue;
            }
        } else if specs.is_flag(OptFlags::OptionsEverywhere) {
            parsed.other.push(opt);
        } else {
            parsed.other.push(opt);
            break;
        }
    }

    loop {
        match iter.next() {
            None => break,
            Some(v) => parsed.other.push(v.clone()),
        }
    }

    parsed
}

const OPTION_TERMINATOR: &str = "--";
const LONG_OPTION_PREFIX: &str = "--";
const SHORT_OPTION_PREFIX: &str = "-";
const INVALID_SHORT_OPTION_CHARS: &str = " -";
const INVALID_LONG_OPTION_CHARS: &str = " =";

fn is_option_terminator(s: &str) -> bool {
    s == OPTION_TERMINATOR
}

fn is_long_option_prefix(s: &str) -> bool {
    if !s.starts_with(LONG_OPTION_PREFIX) {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();
    let prefix_count = LONG_OPTION_PREFIX.chars().count();

    if chars.len() > prefix_count {
        let next = chars[prefix_count];
        if next != '-' {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn get_long_option(s: &str) -> String {
    if !is_long_option_prefix(s) {
        panic!("Not a valid long option {}.", s);
    }
    let chars: Vec<char> = s.chars().collect();
    let prefix_count = LONG_OPTION_PREFIX.chars().count();
    let mut string = String::new();
    for c in &chars[prefix_count..] {
        string.push(*c);
    }
    string
}

fn get_long_option_name(s: &str) -> String {
    let option = get_long_option(s);
    let mut iter = option.split('=');
    match iter.next() {
        None => panic!("Not a valid long option."),
        Some(n) => n.to_string(),
    }
}

fn is_long_option_equal_sign(s: &str) -> bool {
    let option = get_long_option(s);
    let chars: Vec<char> = option.chars().collect();
    for c in &chars[2..] { // Long option name is at least 2 chars long.
        if *c == '=' {
            return true;
        }
    }
    false
}

fn get_long_option_equal_value(s: &str) -> String {
    let option = get_long_option(s);
    let v = option.split_once('=');
    match v {
        None => "".to_string(),
        Some((_, v)) => v.to_string(),
    }
}

pub fn is_valid_long_option_name(s: &str) -> bool {
    if s.starts_with('-') {
        return false;
    }
    for c in INVALID_LONG_OPTION_CHARS.chars() {
        if s.contains(c) {
            return false;
        }
    }
    true
}

pub fn is_valid_short_option_name(s: &str) -> bool {
    if s.chars().count() != 1 {
        return false;
    }
    for c in INVALID_SHORT_OPTION_CHARS.chars() {
        if s.contains(c) {
            return false;
        }
    }
    true
}

fn is_short_option_prefix(s: &str) -> bool {
    if !s.starts_with(SHORT_OPTION_PREFIX) {
        return false;
    }

    let prefix_count = SHORT_OPTION_PREFIX.chars().count();
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 1 + prefix_count {
        return false;
    }

    is_valid_short_option_name(&chars[1].to_string())
}

fn get_short_option_series(s: &str) -> String {
    let prefix_count = SHORT_OPTION_PREFIX.chars().count();
    let chars: Vec<char> = s.chars().collect();
    let mut string = String::new();
    for c in &chars[prefix_count..] {
        string.push(*c);
    }
    string
}
