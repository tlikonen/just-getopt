use crate::{Args, Opt, OptFlags, OptSpecs, OptValueType};

#[cfg(test)]
mod tests;

pub fn parse(specs: &OptSpecs, args: &Vec<String>) -> Args {
    let mut parsed = Args::new();
    let mut iter = args.iter();

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

                match spec_test {
                    None => (),
                    Some(spec) => {
                        let value_required: bool;
                        let value: Option<String>;

                        match spec.value_type {
                            OptValueType::Required => {
                                value_required = true;
                                if is_long_option_equal_sign(&opt) {
                                    value = Some(get_long_option_equal_value(&opt).to_string());
                                } else {
                                    match iter.next() {
                                        None => {
                                            value = None;
                                        }
                                        Some(v) => {
                                            value = Some(v.clone());
                                        }
                                    }
                                }
                            }

                            OptValueType::Optional => {
                                value_required = false;
                                if is_long_option_equal_sign(&opt) {
                                    value = Some(get_long_option_equal_value(&opt).to_string());
                                } else {
                                    value = None;
                                }
                            }

                            OptValueType::None => {
                                value_required = false;
                                value = None;
                                if is_long_option_equal_sign(&opt) {
                                    parsed.unknown.push(format!("{}=", name));
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
            }

            parsed.unknown.push(name);
            continue;
        } else if is_short_option_prefix(&opt) {
            let mut char_iter = get_short_option_series(&opt).chars();

            loop {
                let name = match char_iter.next() {
                    None => break,
                    Some(v) => v.to_string(),
                };

                if is_valid_short_option_name(&name) {
                    match specs.get_short_option_match(&name) {
                        None => (),
                        Some(spec) => {
                            let value_required: bool;
                            let value: Option<String>;

                            match spec.value_type {
                                OptValueType::Required => {
                                    value_required = true;
                                    let chars = char_iter.clone().collect::<String>();
                                    while char_iter.next().is_some() {}
                                    if chars.len() > 0 {
                                        value = Some(chars);
                                    } else {
                                        match iter.next() {
                                            None => {
                                                value = None;
                                            }
                                            Some(v) => {
                                                value = Some(v.to_string());
                                            }
                                        }
                                    }
                                }

                                OptValueType::Optional => {
                                    value_required = false;
                                    let chars = char_iter.clone().collect::<String>();
                                    while char_iter.next().is_some() {}
                                    if chars.len() > 0 {
                                        value = Some(chars);
                                    } else {
                                        value = None;
                                    }
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
                }

                parsed.unknown.push(name);
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
    let len = LONG_OPTION_PREFIX.len();
    if s.len() < 1 + len {
        return false;
    }
    s.starts_with(LONG_OPTION_PREFIX) && &s[len..len + 1] != "-"
}

fn get_long_option(s: &str) -> &str {
    if !is_long_option_prefix(s) {
        panic!("Not a valid long option {}.", s);
    }
    &s[LONG_OPTION_PREFIX.len()..]
}

fn get_long_option_name(s: &str) -> &str {
    let mut iter = get_long_option(&s).split('=');
    iter.next().expect("Not a valid long option.")
}

fn is_long_option_equal_sign(s: &str) -> bool {
    get_long_option(s)[2..].contains('=')
}

fn get_long_option_equal_value(s: &str) -> &str {
    let v = get_long_option(s).split_once('=');
    match v {
        None => "",
        Some((_, v)) => v,
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
    if s.len() != 1 {
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
    let len = SHORT_OPTION_PREFIX.len();
    if s.len() < 1 + len {
        return false;
    }
    s.starts_with(SHORT_OPTION_PREFIX) && is_valid_short_option_name(&s[len..len + 1])
}

fn get_short_option_series(s: &str) -> &str {
    &s[SHORT_OPTION_PREFIX.len()..]
}
