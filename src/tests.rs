use super::*;

#[test]
fn create_optspecs() {
    let mut spec;
    let mut expect;

    spec = OptSpecs::new().option("help", "help", OptValueType::None);
    expect = OptSpec {
        id: String::from("help"),
        name: String::from("help"),
        value_type: OptValueType::None,
    };
    assert_eq!(1, spec.options.len());
    assert_eq!(&expect, &spec.options[0]);

    spec = spec.option("file", "f", OptValueType::Optional);
    expect = OptSpec {
        id: String::from("file"),
        name: String::from("f"),
        value_type: OptValueType::Optional,
    };
    assert_eq!(2, spec.options.len());
    assert_eq!(&expect, &spec.options[1]);

    spec = spec.option("file", "file", OptValueType::Required);
    expect = OptSpec {
        id: String::from("file"),
        name: String::from("file"),
        value_type: OptValueType::Required,
    };
    assert_eq!(3, spec.options.len());
    assert_eq!(&expect, &spec.options[2]);

    spec = spec.flag(OptFlags::OptionsEverywhere);
    assert_eq!(true, spec.is_flag(OptFlags::OptionsEverywhere));
    spec = spec.flag(OptFlags::PrefixMatchLongOptions);
    assert_eq!(true, spec.is_flag(OptFlags::PrefixMatchLongOptions));
}

#[test]
fn check_is_flag() {
    let mut spec = OptSpecs::new().flag(OptFlags::OptionsEverywhere);
    assert_eq!(true, spec.is_flag(OptFlags::OptionsEverywhere));

    spec = spec.flag(OptFlags::PrefixMatchLongOptions);
    assert_eq!(true, spec.is_flag(OptFlags::PrefixMatchLongOptions));
}

#[test]
fn parsed_output_01() {
    let specs = OptSpecs::new()
        .option("help", "h", OptValueType::None)
        .option("help", "help", OptValueType::None)
        .option("file", "f", OptValueType::Required)
        .option("file", "file", OptValueType::Required);

    let parsed = specs.getopt(["-h", "--help", "-f123", "-f", "456", "foo", "bar"]);

    assert_eq!("h", parsed.options_first("help").unwrap().name);
    assert_eq!("help", parsed.options_last("help").unwrap().name);
    assert_eq!("help", parsed.options_first("help").unwrap().id);
    assert_eq!("help", parsed.options_last("help").unwrap().id);
    assert_eq!(false, parsed.options_first("help").unwrap().value_required);
    assert_eq!(false, parsed.options_last("help").unwrap().value_required);

    assert_eq!("f", parsed.options_first("file").unwrap().name);
    assert_eq!(
        "123",
        parsed.options_first("file").unwrap().value.clone().unwrap()
    );
    assert_eq!(
        "456",
        parsed.options_last("file").unwrap().value.clone().unwrap()
    );
    assert_eq!(true, parsed.options_first("file").unwrap().value_required);

    assert_eq!("foo", parsed.other[0]);
    assert_eq!("bar", parsed.other[1]);
}

#[test]
fn parsed_output_02() {
    let specs = OptSpecs::new().option("help", "h", OptValueType::None);

    let parsed = specs.getopt(["-h", "foo", "-h"]);

    assert_eq!("h", parsed.options_first("help").unwrap().name);
    assert_eq!("foo", parsed.other[0]);
    assert_eq!("-h", parsed.other[1]);
}

#[test]
fn parsed_output_03() {
    let specs = OptSpecs::new()
        .flag(OptFlags::OptionsEverywhere)
        .option("help", "h", OptValueType::None)
        .option("help", "help", OptValueType::None)
        .option("file", "f", OptValueType::Required)
        .option("file", "file", OptValueType::Required);

    let parsed = specs.getopt(["-h", "foo", "--help", "--file=123", "bar", "--file", "456"]);

    assert_eq!("h", parsed.options_first("help").unwrap().name);
    assert_eq!("help", parsed.options_last("help").unwrap().name);
    assert_eq!(
        "123",
        parsed.options_first("file").unwrap().value.clone().unwrap()
    );
    assert_eq!(
        "456",
        parsed.options_last("file").unwrap().value.clone().unwrap()
    );
    assert_eq!("foo", parsed.other[0]);
    assert_eq!("bar", parsed.other[1]);
}

#[test]
fn parsed_output_04() {
    let specs = OptSpecs::new()
        .option("debug", "d", OptValueType::Optional)
        .option("verbose", "verbose", OptValueType::Optional);

    let parsed = specs.getopt(["-d1", "-d", "--verbose", "--verbose=123"]);

    assert_eq!(
        "1",
        parsed
            .options_first("debug")
            .unwrap()
            .value
            .clone()
            .unwrap()
    );
    assert_eq!(true, parsed.options_last("debug").unwrap().value.is_none());
    assert_eq!(false, parsed.options_last("debug").unwrap().value_required);

    assert_eq!(
        true,
        parsed.options_first("verbose").unwrap().value.is_none()
    );
    assert_eq!(
        "123",
        parsed
            .options_last("verbose")
            .unwrap()
            .value
            .clone()
            .unwrap()
    );
    assert_eq!(
        false,
        parsed.options_last("verbose").unwrap().value_required
    );
}

#[test]
fn parsed_output_05() {
    let specs = OptSpecs::new().option("debug", "d", OptValueType::Optional);

    let parsed = specs.getopt(["-abcd", "-adbc"]);

    assert_eq!(true, parsed.options_first("debug").unwrap().value.is_none());
    assert_eq!(
        "bc",
        parsed.options_last("debug").unwrap().value.clone().unwrap()
    );

    assert_eq!("a", parsed.unknown[0]);
    assert_eq!("b", parsed.unknown[1]);
    assert_eq!("c", parsed.unknown[2]);
}

#[test]
fn parsed_output_06() {
    let specs = OptSpecs::new()
        .option("aaa", "bbb", OptValueType::None)
        .option("aaa", "c", OptValueType::None)
        .option("aaa", "d", OptValueType::None)
        .option("aaa", "eee", OptValueType::None);

    let parsed = specs.getopt(["--bbb", "-cd", "--eee"]);

    let m = parsed.options_all("aaa");
    assert_eq!("bbb", m[0].name);
    assert_eq!("c", m[1].name);
    assert_eq!("d", m[2].name);
    assert_eq!("eee", m[3].name);
}

#[test]
fn parsed_output_07() {
    let specs = OptSpecs::new()
        .flag(OptFlags::PrefixMatchLongOptions)
        .option("version", "version", OptValueType::None)
        .option("verbose", "verbose", OptValueType::None);

    let parsed = specs.getopt(["--ver", "--verb", "--versi", "--verbose"]);

    assert_eq!("ver", parsed.unknown[0]);
    assert_eq!("verb", parsed.options_first("verbose").unwrap().name);
    assert_eq!("verbose", parsed.options_last("verbose").unwrap().name);
    assert_eq!("version", parsed.options_first("version").unwrap().id);
    assert_eq!("versi", parsed.options_first("version").unwrap().name);
}

#[test]
fn parsed_output_08() {
    let specs = OptSpecs::new()
        // .flag(OptFlags::PrefixMatchLongOptions) Must be commented!
        .option("version", "version", OptValueType::None)
        .option("verbose", "verbose", OptValueType::None);

    let parsed = specs.getopt(["--version", "--ver", "--verb", "--versi", "--verbose"]);

    assert_eq!("ver", parsed.unknown[0]);
    assert_eq!("verb", parsed.unknown[1]);
    assert_eq!("versi", parsed.unknown[2]);
    assert_eq!("version", parsed.options_first("version").unwrap().name);
    assert_eq!("verbose", parsed.options_first("verbose").unwrap().name);
}

#[test]
fn parsed_output_09() {
    let specs = OptSpecs::new()
        .flag(OptFlags::OptionsEverywhere)
        .option("help", "h", OptValueType::None)
        .option("file", "file", OptValueType::Required);

    let parsed = specs.getopt(["-h", "foo", "--file=123", "--", "bar", "--file", "456"]);

    assert_eq!("h", parsed.options_first("help").unwrap().name);
    assert_eq!("file", parsed.options_first("file").unwrap().name);
    assert_eq!(
        "123",
        parsed.options_first("file").unwrap().value.clone().unwrap()
    );

    assert_eq!("foo", parsed.other[0]);
    assert_eq!("bar", parsed.other[1]);
    assert_eq!("--file", parsed.other[2]);
    assert_eq!("456", parsed.other[3]);
}

#[test]
fn parsed_output_10() {
    let specs = OptSpecs::new().option("file", "file", OptValueType::Required);

    let parsed = specs.getopt(["--file=", "--file"]);

    assert_eq!(true, parsed.options_first("file").unwrap().value_required);
    assert_eq!(
        "",
        parsed.options_first("file").unwrap().value.clone().unwrap()
    );
    assert_eq!(true, parsed.options_last("file").unwrap().value.is_none());
}

#[test]
fn parsed_output_11() {
    let specs = OptSpecs::new()
        .option("file", "f", OptValueType::Required)
        .option("debug", "d", OptValueType::Required);

    let parsed = specs.getopt(["-fx", "-d", "", "-f"]);

    assert_eq!(true, parsed.options_first("file").unwrap().value_required);
    assert_eq!(
        "x",
        parsed.options_first("file").unwrap().value.clone().unwrap()
    );
    assert_eq!(true, parsed.options_last("file").unwrap().value.is_none());
    assert_eq!(
        "",
        parsed
            .options_first("debug")
            .unwrap()
            .value
            .clone()
            .unwrap()
    );
}

#[test]
fn parsed_output_12() {
    let specs = OptSpecs::new()
        .option("file", "f", OptValueType::Required)
        .option("debug", "d", OptValueType::Required);

    let parsed = specs.getopt(["-f123", "-d", "", "-f", "456", "-f"]);

    let f = parsed.options_value_all("file");
    let d = parsed.options_value_all("debug");

    assert_eq!(2, f.len());
    assert_eq!("123", f[0]);
    assert_eq!("456", f[1]);

    assert_eq!(1, d.len());
    assert_eq!("", d[0]);

    assert_eq!(true, parsed.options_last("file").unwrap().value.is_none());
}

#[test]
fn parsed_output_13() {
    let specs = OptSpecs::new()
        .option("file", "file", OptValueType::Required)
        .option("debug", "debug", OptValueType::Required);

    let parsed = specs.getopt(["--file=123", "--debug", "", "--file", "456", "--file"]);

    let f = parsed.options_value_all("file");
    let d = parsed.options_value_all("debug");

    assert_eq!(2, f.len());
    assert_eq!("123", f[0]);
    assert_eq!("456", f[1]);

    assert_eq!(1, d.len());
    assert_eq!("", d[0]);

    assert_eq!(true, parsed.options_last("file").unwrap().value.is_none());
}

#[test]
fn parsed_output_14() {
    let specs = OptSpecs::new()
        .flag(OptFlags::OptionsEverywhere)
        .option("debug", "d", OptValueType::Optional)
        .option("debug", "debug", OptValueType::Optional);

    let parsed = specs.getopt([
        "-d",
        "-d123",
        "-d",
        "--debug",
        "--debug=",
        "foo",
        "--debug=456",
        "-d",
    ]);

    let d = parsed.options_all("debug");
    assert_eq!(7, d.len());

    let d = parsed.options_value_all("debug");
    assert_eq!(3, d.len());
    assert_eq!("123", d[0]);
    assert_eq!("", d[1]);
    assert_eq!("456", d[2]);
    assert_eq!("123", parsed.options_value_first("debug").unwrap());
    assert_eq!("456", parsed.options_value_last("debug").unwrap());

    assert_eq!("foo", parsed.other[0]);
}

#[test]
fn parsed_output_15() {
    let specs = OptSpecs::new();
    let parsed = specs.getopt([
        "-abcd",
        "-e",
        "--debug",
        "--",
        "--debug=",
        "foo",
        "--debug=456",
    ]);

    assert_eq!(0, parsed.options.len());
    assert_eq!(3, parsed.other.len());
    assert_eq!(6, parsed.unknown.len());
}

#[test]
fn parsed_output_16() {
    let specs = OptSpecs::new().option("file", "file", OptValueType::Required);

    let parsed = specs.getopt(["--file", "--", "--", "--"]);

    assert_eq!(
        "--",
        parsed.options_first("file").unwrap().value.clone().unwrap()
    );
    assert_eq!(1, parsed.other.len());
    assert_eq!("--", parsed.other[0]);
}

#[test]
fn parsed_output_17() {
    let parsed = OptSpecs::new().getopt(["foo", "bar"]);

    assert_eq!(None, parsed.options_first("not-at-all"));
    assert_eq!(None, parsed.options_last("not-at-all"));
}

#[test]
fn parsed_output_18() {
    let parsed = OptSpecs::new()
        .option("bar", "bar", OptValueType::None)
        .getopt(["-aaa", "--foo", "--foo", "--bar=", "--bar="]);

    assert_eq!(3, parsed.unknown.len());
    assert_eq!("a", parsed.unknown[0]);
    assert_eq!("foo", parsed.unknown[1]);
    assert_eq!("bar=", parsed.unknown[2]);
}
