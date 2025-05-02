use crate::{Args, Opt, OptFlags, OptSpecs, OptValue};
use alloc::{
    format,
    string::{String, ToString},
};

pub fn parse<I>(specs: &OptSpecs, mut iter: I) -> Args
where
    I: Iterator<Item = String>,
{
    let mut parsed = Args::new();
    let mut option_count: u32 = 0;
    let mut other_count: u32 = 0;
    let mut unknown_count: u32 = 0;

    loop {
        if option_count >= specs.option_limit
            && other_count >= specs.other_limit
            && unknown_count >= specs.unknown_limit
        {
            break;
        }

        let arg = match iter.next() {
            None => break,
            Some(s) => s,
        };

        if is_option_terminator(&arg) {
            break;
        } else if is_long_option_prefix(&arg) {
            let name = get_long_option_name(&arg);

            if is_valid_long_option_name(&name) {
                let opt_match = if specs.is_flag(OptFlags::PrefixMatchLongOptions) {
                    specs.get_long_option_prefix_match(&name)
                } else {
                    specs.get_long_option_match(&name)
                };

                if let Some(spec) = opt_match {
                    let value_required: bool;
                    let mut value: Option<String>;

                    match spec.value_type {
                        OptValue::Required | OptValue::RequiredNonEmpty => {
                            value_required = true;
                            value = if is_long_option_equal_sign(&arg) {
                                Some(get_long_option_equal_value(&arg))
                            } else {
                                iter.next()
                            };
                        }

                        OptValue::Optional | OptValue::OptionalNonEmpty => {
                            value_required = false;
                            value = if is_long_option_equal_sign(&arg) {
                                Some(get_long_option_equal_value(&arg))
                            } else {
                                None
                            };
                        }

                        OptValue::None => {
                            value_required = false;
                            value = None;
                            if is_long_option_equal_sign(&arg) {
                                let n = format!("{}=", name);
                                if unknown_count < specs.unknown_limit
                                    && !parsed.unknown.contains(&n)
                                {
                                    parsed.unknown.push(n);
                                    unknown_count += 1;
                                }
                                continue;
                            }
                        }
                    }

                    if option_count < specs.option_limit {
                        match spec.value_type {
                            OptValue::RequiredNonEmpty | OptValue::OptionalNonEmpty => {
                                value = value.filter(|v| !v.is_empty());
                            }
                            _ => (),
                        }

                        parsed.options.push(Opt {
                            id: spec.id.clone(),
                            name,
                            value_required,
                            value,
                        });
                        option_count += 1;
                    }
                    continue;
                }
            }

            if unknown_count < specs.unknown_limit && !parsed.unknown.contains(&name) {
                parsed.unknown.push(name);
                unknown_count += 1;
            }
            continue;
        } else if is_short_option_prefix(&arg) {
            let series = get_short_option_series(&arg);
            let mut char_iter = series.chars();

            loop {
                let name = match char_iter.next() {
                    None => break,
                    Some(c) => c.to_string(),
                };

                if is_valid_short_option_name(&name) {
                    if let Some(spec) = specs.get_short_option_match(&name) {
                        let value_required: bool;
                        let mut value: Option<String>;

                        match spec.value_type {
                            OptValue::Required | OptValue::RequiredNonEmpty => {
                                value_required = true;
                                let mut chars = String::with_capacity(5);
                                for c in char_iter.by_ref() {
                                    chars.push(c);
                                }
                                value = match chars.chars().count() {
                                    0 => iter.next(),
                                    _ => Some(chars),
                                };
                            }

                            OptValue::Optional | OptValue::OptionalNonEmpty => {
                                value_required = false;
                                let mut chars = String::with_capacity(5);
                                for c in char_iter.by_ref() {
                                    chars.push(c);
                                }
                                value = match chars.chars().count() {
                                    0 => None,
                                    _ => Some(chars),
                                };
                            }

                            OptValue::None => {
                                value_required = false;
                                value = None;
                            }
                        }

                        if option_count < specs.option_limit {
                            match spec.value_type {
                                OptValue::RequiredNonEmpty | OptValue::OptionalNonEmpty => {
                                    value = value.filter(|v| !v.is_empty());
                                }
                                _ => (),
                            }

                            parsed.options.push(Opt {
                                id: spec.id.clone(),
                                name,
                                value_required,
                                value,
                            });
                            option_count += 1;
                        }
                        continue;
                    }
                }

                if unknown_count < specs.unknown_limit && !parsed.unknown.contains(&name) {
                    parsed.unknown.push(name);
                    unknown_count += 1;
                }
                continue;
            }
        } else {
            if other_count < specs.other_limit {
                parsed.other.push(arg);
                other_count += 1;
            }
            if !specs.is_flag(OptFlags::OptionsEverywhere) {
                break;
            }
        }
    }

    loop {
        if other_count >= specs.other_limit {
            break;
        }

        match iter.next() {
            None => break,
            Some(s) => {
                if other_count < specs.other_limit {
                    parsed.other.push(s);
                    other_count += 1;
                }
            }
        }
    }

    parsed
}

const OPTION_TERMINATOR: &str = "--";
const LONG_OPTION_PREFIX: &str = "--";
const LONG_OPTION_PREFIX_COUNT: usize = 2;
const LONG_OPTION_NAME_MIN_COUNT: usize = 2;
const SHORT_OPTION_PREFIX: &str = "-";
const SHORT_OPTION_PREFIX_COUNT: usize = 1;
const INVALID_SHORT_OPTION_CHARS: &str = " -";
const INVALID_LONG_OPTION_CHARS: &str = " =";

fn is_option_terminator(s: &str) -> bool {
    s == OPTION_TERMINATOR
}

fn is_long_option_prefix(s: &str) -> bool {
    s.starts_with(LONG_OPTION_PREFIX)
        && s.chars()
            .nth(LONG_OPTION_PREFIX_COUNT)
            .map_or(false, |c| c != '-')
}

fn get_long_option(s: &str) -> String {
    if !is_long_option_prefix(s) {
        panic!("Not a valid long option {}.", s);
    }
    s.chars().skip(LONG_OPTION_PREFIX_COUNT).collect()
}

fn get_long_option_name(s: &str) -> String {
    get_long_option(s).split('=').next().unwrap().to_string()
}

fn is_long_option_equal_sign(s: &str) -> bool {
    get_long_option(s)
        .chars()
        .skip(LONG_OPTION_NAME_MIN_COUNT)
        .any(|c| c == '=')
}

fn get_long_option_equal_value(s: &str) -> String {
    get_long_option(s)
        .split_once('=')
        .map_or_else(|| "", |(_, v)| v)
        .to_string()
}

pub fn is_valid_long_option_name(s: &str) -> bool {
    !s.starts_with('-')
        && s.chars().nth(LONG_OPTION_NAME_MIN_COUNT - 1).is_some()
        && !s.chars().any(|c| INVALID_LONG_OPTION_CHARS.contains(c))
}

pub fn is_valid_short_option_name(s: &str) -> bool {
    s.chars().count() == 1 && !INVALID_SHORT_OPTION_CHARS.contains(s)
}

fn is_short_option_prefix(s: &str) -> bool {
    s.starts_with(SHORT_OPTION_PREFIX)
        && s.chars()
            .nth(SHORT_OPTION_PREFIX_COUNT)
            .map_or(false, |c| !INVALID_SHORT_OPTION_CHARS.contains(c))
    // Rust 1.70:
    // .is_some_and(|c| !INVALID_SHORT_OPTION_CHARS.contains(c))
}

fn get_short_option_series(s: &str) -> String {
    s.chars().skip(SHORT_OPTION_PREFIX_COUNT).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_is_long_option_prefix() {
        assert_eq!(true, is_long_option_prefix("--ab"));
        assert_eq!(true, is_long_option_prefix("--abc"));
        assert_eq!(true, is_long_option_prefix("--a"));
        assert_eq!(true, is_long_option_prefix("--ä"));
        assert_eq!(false, is_long_option_prefix("---ab"));
        assert_eq!(false, is_long_option_prefix("---"));
        assert_eq!(false, is_long_option_prefix(""));
        assert_eq!(false, is_long_option_prefix(" "));
        assert_eq!(false, is_long_option_prefix("-x"));
        assert_eq!(false, is_long_option_prefix("--"));
        assert_eq!(false, is_long_option_prefix("-"));
    }

    #[test]
    fn t_get_long_option() {
        assert_eq!("abc", get_long_option("--abc"));
        assert_eq!("ab", get_long_option("--ab"));
        assert_eq!("abc=", get_long_option("--abc="));
        assert_eq!("abc=foo", get_long_option("--abc=foo"));
        assert_eq!("ä€o=foo", get_long_option("--ä€o=foo"));
    }

    #[test]
    #[should_panic]
    fn t_get_long_option_panic() {
        get_long_option("-");
        get_long_option("--");
        get_long_option("--a");
    }

    #[test]
    fn t_get_long_option_name() {
        assert_eq!("abc", get_long_option_name("--abc"));
        assert_eq!("ä€", get_long_option_name("--ä€"));
        assert_eq!("abc", get_long_option_name("--abc="));
        assert_eq!("abc", get_long_option_name("--abc=1"));
        assert_eq!("abc", get_long_option_name("--abc=134"));
        assert_eq!("abc", get_long_option_name("--abc=134="));
        assert_eq!("abc", get_long_option_name("--abc=134=123"));
        assert_eq!("abc-def", get_long_option_name("--abc-def=  "));
        assert_eq!("abc-ä€", get_long_option_name("--abc-ä€=  "));
    }

    #[test]
    fn t_is_long_option_equal_sign() {
        assert_eq!(true, is_long_option_equal_sign("--abc="));
        assert_eq!(true, is_long_option_equal_sign("--ab="));
        assert_eq!(true, is_long_option_equal_sign("--ab=1"));
        assert_eq!(true, is_long_option_equal_sign("--ab=123"));
        assert_eq!(true, is_long_option_equal_sign("--ä€=123"));
        assert_eq!(true, is_long_option_equal_sign("--ab=123=123"));
        assert_eq!(false, is_long_option_equal_sign("--abcd"));
        assert_eq!(false, is_long_option_equal_sign("--ab"));
        assert_eq!(false, is_long_option_equal_sign("--a="));
    }

    #[test]
    fn t_get_long_option_equal_value() {
        assert_eq!("", get_long_option_equal_value("--abc"));
        assert_eq!("", get_long_option_equal_value("--abc="));
        assert_eq!("1", get_long_option_equal_value("--abc=1"));
        assert_eq!("=", get_long_option_equal_value("--abc=="));
        assert_eq!("--", get_long_option_equal_value("--abc=--"));
        assert_eq!("123", get_long_option_equal_value("--abc=123"));
        assert_eq!(" 12 3 ", get_long_option_equal_value("--abc= 12 3 "));
        assert_eq!("123=123=", get_long_option_equal_value("--abc=123=123="));
        assert_eq!("!", get_long_option_equal_value("--abc-def=!"));
        assert_eq!("!", get_long_option_equal_value("--abc-ä€=!"));
        assert_eq!("öOö", get_long_option_equal_value("--abc-ä€=öOö"));
    }

    #[test]
    fn t_is_valid_long_option_name() {
        assert_eq!(true, is_valid_long_option_name("ab"));
        assert_eq!(true, is_valid_long_option_name("ab-"));
        assert_eq!(true, is_valid_long_option_name("ab-abc"));
        assert_eq!(true, is_valid_long_option_name("ä€"));
        assert_eq!(false, is_valid_long_option_name("a"));
        assert_eq!(false, is_valid_long_option_name("€"));
        assert_eq!(false, is_valid_long_option_name("-abc"));
        assert_eq!(false, is_valid_long_option_name("abc="));
        assert_eq!(false, is_valid_long_option_name("abc "));
        assert_eq!(false, is_valid_long_option_name(" abc "));
        assert_eq!(false, is_valid_long_option_name("abc ab"));
    }

    #[test]
    fn t_is_valid_short_option_name() {
        assert_eq!(true, is_valid_short_option_name("a"));
        assert_eq!(true, is_valid_short_option_name("ä"));
        assert_eq!(true, is_valid_short_option_name("€"));
        assert_eq!(true, is_valid_short_option_name("1"));
        assert_eq!(true, is_valid_short_option_name("?"));
        assert_eq!(true, is_valid_short_option_name("="));
        assert_eq!(true, is_valid_short_option_name("%"));
        assert_eq!(false, is_valid_short_option_name("-"));
        assert_eq!(false, is_valid_short_option_name(" "));
    }

    #[test]
    fn t_is_short_option_prefix() {
        assert_eq!(true, is_short_option_prefix("-a"));
        assert_eq!(true, is_short_option_prefix("-ä"));
        assert_eq!(true, is_short_option_prefix("-€"));
        assert_eq!(true, is_short_option_prefix("-="));
        assert_eq!(true, is_short_option_prefix("-?"));
        assert_eq!(true, is_short_option_prefix("-abcd"));
        assert_eq!(false, is_short_option_prefix("-"));
        assert_eq!(false, is_short_option_prefix("--"));
        assert_eq!(false, is_short_option_prefix("a"));
        assert_eq!(false, is_short_option_prefix("aa"));
        assert_eq!(false, is_short_option_prefix("aaa"));
        assert_eq!(false, is_short_option_prefix(""));
        assert_eq!(false, is_short_option_prefix(" "));
        assert_eq!(false, is_short_option_prefix("- "));
        assert_eq!(false, is_short_option_prefix("--ab"));
        assert_eq!(false, is_short_option_prefix("--a"));
    }

    #[test]
    fn t_get_short_option_series() {
        assert_eq!("a", get_short_option_series("-a"));
        assert_eq!("ab", get_short_option_series("-ab"));
        assert_eq!("ä€", get_short_option_series("-ä€"));
        assert_eq!("ab -", get_short_option_series("-ab -"));
    }

    #[test]
    fn t_get_short_option_match() {
        let spec = OptSpecs::new()
            .option("help", "help", OptValue::None)
            .option("verbose", "verbose", OptValue::None)
            .option("verbose", "v", OptValue::None)
            .option("€uro", "€", OptValue::None)
            .option("file", "f", OptValue::None);

        {
            let m = &spec.get_short_option_match("v");
            assert!(m.is_some());
            let m = m.unwrap();
            assert_eq!("verbose", m.id);
            assert_eq!("v", m.name);
            assert_eq!(OptValue::None, m.value_type);
        }

        {
            let m = &spec.get_short_option_match("f");
            assert!(m.is_some());
            let m = m.unwrap();
            assert_eq!("file", m.id);
            assert_eq!("f", m.name);
            assert_eq!(OptValue::None, m.value_type);
        }

        {
            let m = &spec.get_short_option_match("€");
            assert!(m.is_some());
            let m = m.unwrap();
            assert_eq!("€uro", m.id);
            assert_eq!("€", m.name);
            assert_eq!(OptValue::None, m.value_type);
        }

        {
            let m = &spec.get_short_option_match("x");
            assert!(m.is_none());
        }
    }

    #[test]
    fn t_get_long_option_match() {
        let spec = OptSpecs::new()
            .option("help", "help", OptValue::None)
            .option("verbose", "verbose", OptValue::None)
            .option("verbose", "v", OptValue::None)
            .option("€uro", "€uro", OptValue::None)
            .option("file", "f", OptValue::None);

        {
            let m = &spec.get_long_option_match("verbose");
            assert!(m.is_some());
            let v = &m.unwrap();
            assert_eq!("verbose", v.id);
            assert_eq!("verbose", v.name);
            assert_eq!(OptValue::None, v.value_type);
        }

        {
            let m = &spec.get_long_option_match("help");
            assert!(m.is_some());
            let v = &m.unwrap();
            assert_eq!("help", v.id);
            assert_eq!("help", v.name);
            assert_eq!(OptValue::None, v.value_type);
        }

        {
            let m = &spec.get_long_option_match("€uro");
            assert!(m.is_some());
            let v = &m.unwrap();
            assert_eq!("€uro", v.id);
            assert_eq!("€uro", v.name);
            assert_eq!(OptValue::None, v.value_type);
        }

        {
            let m = &spec.get_long_option_match("asdf");
            assert!(m.is_none());
        }
    }

    #[test]
    fn t_get_long_option_prefix_match() {
        use crate::OptSpec;

        let spec = OptSpecs::new()
            .option("foo", "foo-option", OptValue::None)
            .option("bar", "foo-€ö-option", OptValue::None)
            .option("verbose", "verbose", OptValue::None)
            .option("version", "version", OptValue::None);

        assert_eq!(true, spec.get_long_option_prefix_match("ver").is_none());
        assert_eq!(true, spec.get_long_option_prefix_match("foo-").is_none());
        assert_eq!(
            true,
            spec.get_long_option_prefix_match("not-at-all").is_none()
        );

        {
            let m = &spec.get_long_option_prefix_match("verb");
            match m {
                Some(OptSpec { id: i, name: n, .. }) => {
                    assert_eq!("verbose", i);
                    assert_eq!("verbose", n);
                }
                None => panic!("Should not panic!"),
            };
        }

        {
            let m = &spec.get_long_option_prefix_match("foo-o");
            match m {
                Some(OptSpec { id: i, name: n, .. }) => {
                    assert_eq!("foo", i);
                    assert_eq!("foo-option", n);
                }
                None => panic!("Should not panic!"),
            };
        }

        {
            let m = &spec.get_long_option_prefix_match("foo-€");
            match m {
                Some(OptSpec { id: i, name: n, .. }) => {
                    assert_eq!("bar", i);
                    assert_eq!("foo-€ö-option", n);
                }
                None => panic!("Should not panic!"),
            };
        }

        {
            let m = &spec.get_long_option_prefix_match("version");
            match m {
                Some(OptSpec { id: i, name: n, .. }) => {
                    assert_eq!("version", i);
                    assert_eq!("version", n);
                }
                None => panic!("Should not panic!"),
            };
        }
    }
}
