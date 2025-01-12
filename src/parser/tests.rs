use super::*;

#[test]
fn check_is_long_option_prefix() {
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
fn check_get_long_option() {
    assert_eq!("abc", get_long_option("--abc"));
    assert_eq!("ab", get_long_option("--ab"));
    assert_eq!("abc=", get_long_option("--abc="));
    assert_eq!("abc=foo", get_long_option("--abc=foo"));
    assert_eq!("ä€o=foo", get_long_option("--ä€o=foo"));
}

#[test]
#[should_panic]
fn check_get_long_option_panic() {
    get_long_option("-");
    get_long_option("--");
    get_long_option("--a");
}

#[test]
fn check_get_long_option_name() {
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
fn check_is_long_option_equal_sign() {
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
fn check_get_long_option_equal_value() {
    assert_eq!("", get_long_option_equal_value("--abc"));
    assert_eq!("", get_long_option_equal_value("--abc="));
    assert_eq!("1", get_long_option_equal_value("--abc=1"));
    assert_eq!("=", get_long_option_equal_value("--abc=="));
    assert_eq!("123", get_long_option_equal_value("--abc=123"));
    assert_eq!(" 12 3 ", get_long_option_equal_value("--abc= 12 3 "));
    assert_eq!("123=123=", get_long_option_equal_value("--abc=123=123="));
    assert_eq!("!", get_long_option_equal_value("--abc-def=!"));
    assert_eq!("!", get_long_option_equal_value("--abc-ä€=!"));
    assert_eq!("öOö", get_long_option_equal_value("--abc-ä€=öOö"));
}

#[test]
fn check_is_valid_long_option_name() {
    assert_eq!(true, is_valid_long_option_name("ab"));
    assert_eq!(true, is_valid_long_option_name("ab-"));
    assert_eq!(true, is_valid_long_option_name("ab-abc"));
    assert_eq!(true, is_valid_long_option_name("ä€"));
    assert_eq!(false, is_valid_long_option_name("-abc"));
    assert_eq!(false, is_valid_long_option_name("abc="));
    assert_eq!(false, is_valid_long_option_name("abc "));
    assert_eq!(false, is_valid_long_option_name(" abc "));
    assert_eq!(false, is_valid_long_option_name("abc ab"));
}

#[test]
fn check_is_valid_short_option_name() {
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
fn check_is_short_option_prefix() {
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
fn check_get_short_option_series() {
    assert_eq!("a", get_short_option_series("-a"));
    assert_eq!("ab", get_short_option_series("-ab"));
    assert_eq!("ä€", get_short_option_series("-ä€"));
    assert_eq!("ab -", get_short_option_series("-ab -"));
}

#[test]
fn check_get_short_option_match() {
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
fn check_get_long_option_match() {
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
fn check_get_long_option_prefix_matches() {
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
