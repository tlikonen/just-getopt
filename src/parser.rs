use crate::{Args, Opt, OptFlags, OptSpecs, OptValueType};

pub fn parse<I>(specs: &OptSpecs, mut iter: I) -> Args
where
    I: Iterator<Item = String>,
{
    let mut parsed = Args::new();
    let mut args_count: u32 = 0;

    loop {
        if !specs.is_under_limit(args_count) {
            break;
        }

        let opt = match iter.next() {
            None => break,
            Some(v) => {
                args_count += 1;
                v.clone()
            }
        };

        if is_option_terminator(&opt) {
            break;
        } else if is_long_option_prefix(&opt) {
            let name = get_long_option_name(&opt).to_string();

            if is_valid_long_option_name(&name) {
                let opt_match = if specs.is_flag(OptFlags::PrefixMatchLongOptions) {
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

                if let Some(spec) = opt_match {
                    let value_required: bool;
                    let value: Option<String>;

                    match spec.value_type {
                        OptValueType::Required => {
                            value_required = true;
                            value = if is_long_option_equal_sign(&opt) {
                                Some(get_long_option_equal_value(&opt).to_string())
                            } else if specs.is_under_limit(args_count) {
                                match iter.next() {
                                    Some(v) => {
                                        args_count += 1;
                                        Some(v)
                                    }
                                    None => None,
                                }
                            } else {
                                None
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
                        name,
                        value_required,
                        value,
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
                                for c in char_iter.by_ref() {
                                    chars.push(c);
                                }
                                value = if chars.chars().count() > 0 {
                                    Some(chars)
                                } else if specs.is_under_limit(args_count) {
                                    match iter.next() {
                                        Some(v) => {
                                            args_count += 1;
                                            Some(v)
                                        }
                                        None => None,
                                    }
                                } else {
                                    None
                                }
                            }

                            OptValueType::Optional => {
                                value_required = false;
                                let mut chars = String::new();
                                for c in char_iter.by_ref() {
                                    chars.push(c);
                                }
                                value = if chars.chars().count() > 0 {
                                    Some(chars)
                                } else {
                                    None
                                }
                            }

                            OptValueType::None => {
                                value_required = false;
                                value = None;
                            }
                        }

                        parsed.options.push(Opt {
                            id: spec.id.clone(),
                            name,
                            value_required,
                            value,
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
        if !specs.is_under_limit(args_count) {
            if iter.next().is_some() {
                parsed.arg_limit_exceeded = true;
            }
            break;
        }

        match iter.next() {
            None => break,
            Some(v) => {
                args_count += 1;
                parsed.other.push(v.clone());
            }
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
        next != '-'
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
    for c in &chars[2..] {
        // Long option name is at least 2 chars long.
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
    if s.starts_with('-') || s.chars().count() < 2 {
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

    is_valid_short_option_name(&chars[prefix_count].to_string())
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
            .option("help", "help", OptValueType::None)
            .option("verbose", "verbose", OptValueType::None)
            .option("verbose", "v", OptValueType::None)
            .option("€uro", "€", OptValueType::None)
            .option("file", "f", OptValueType::None);

        {
            let m = &spec.get_short_option_match("v");
            assert!(m.is_some());
            let m = m.unwrap();
            assert_eq!("verbose", m.id);
            assert_eq!("v", m.name);
            assert_eq!(OptValueType::None, m.value_type);
        }

        {
            let m = &spec.get_short_option_match("f");
            assert!(m.is_some());
            let m = m.unwrap();
            assert_eq!("file", m.id);
            assert_eq!("f", m.name);
            assert_eq!(OptValueType::None, m.value_type);
        }

        {
            let m = &spec.get_short_option_match("€");
            assert!(m.is_some());
            let m = m.unwrap();
            assert_eq!("€uro", m.id);
            assert_eq!("€", m.name);
            assert_eq!(OptValueType::None, m.value_type);
        }

        {
            let m = &spec.get_short_option_match("x");
            assert!(m.is_none());
        }
    }

    #[test]
    fn t_get_long_option_match() {
        let spec = OptSpecs::new()
            .option("help", "help", OptValueType::None)
            .option("verbose", "verbose", OptValueType::None)
            .option("verbose", "v", OptValueType::None)
            .option("€uro", "€uro", OptValueType::None)
            .option("file", "f", OptValueType::None);

        {
            let m = &spec.get_long_option_match("verbose");
            assert!(m.is_some());
            let v = &m.unwrap();
            assert_eq!("verbose", v.id);
            assert_eq!("verbose", v.name);
            assert_eq!(OptValueType::None, v.value_type);
        }

        {
            let m = &spec.get_long_option_match("help");
            assert!(m.is_some());
            let v = &m.unwrap();
            assert_eq!("help", v.id);
            assert_eq!("help", v.name);
            assert_eq!(OptValueType::None, v.value_type);
        }

        {
            let m = &spec.get_long_option_match("€uro");
            assert!(m.is_some());
            let v = &m.unwrap();
            assert_eq!("€uro", v.id);
            assert_eq!("€uro", v.name);
            assert_eq!(OptValueType::None, v.value_type);
        }

        {
            let m = &spec.get_long_option_match("asdf");
            assert!(m.is_none());
        }
    }

    #[test]
    fn t_get_long_option_prefix_matches() {
        let spec = OptSpecs::new()
            .option("foo", "foo-option", OptValueType::None)
            .option("bar", "foo-€ö-option", OptValueType::None)
            .option("verbose", "verbose", OptValueType::None)
            .option("version", "version", OptValueType::None);

        {
            let m = &spec.get_long_option_prefix_matches("ver");
            match m {
                Some(n) => assert_eq!(2, n.len()),
                None => panic!("Should not panic!"),
            };
        }

        {
            let m = &spec.get_long_option_prefix_matches("verb");
            match m {
                Some(n) => assert_eq!(1, n.len()),
                None => panic!("Should not panic!"),
            };
        }

        {
            let m = &spec.get_long_option_prefix_matches("foo-");
            match m {
                Some(n) => assert_eq!(2, n.len()),
                None => panic!("Should not panic!"),
            };
        }

        {
            let m = &spec.get_long_option_prefix_matches("foo-€");
            match m {
                Some(n) => assert_eq!(1, n.len()),
                None => panic!("Should not panic!"),
            };
        }

        {
            let m = &spec.get_long_option_prefix_matches("version");
            match m {
                Some(n) => assert_eq!(1, n.len()),
                None => panic!("Should not panic!"),
            };
        }

        {
            let m = &spec.get_long_option_prefix_matches("not-at-all");
            assert!(m.is_none());
        }
    }
}
