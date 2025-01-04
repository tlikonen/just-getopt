use just_getopt::{OptSpecs, OptValueType, OptFlags};
use std::process::ExitCode;

fn main() -> ExitCode {
    // Create new option specification struct `OptSpecs` with three
    // different options for logical meanings "help", "file" and
    // "verbose". All three options can be accessed in command-line with
    // a short variant (like "-h") and a long variant (like "--help").
    // Some options accept or require a value.
    //
    // Flag OptionsEverywhere changes parser's behavior. This flag means
    // that options and other arguments can be mixed in their order in
    // the command line. Without the flag the option parsing stops at
    // the first non-option argument and the rest of the command-line is
    // parsed as non-options.
    let specs = OptSpecs::new()
        .flag(OptFlags::OptionsEverywhere)
        .option("help", "h", OptValueType::None)
        .option("help", "help", OptValueType::None)
        .option("file", "f", OptValueType::Required)
        .option("file", "file", OptValueType::Required)
        .option("verbose", "v", OptValueType::Optional)
        .option("verbose", "verbose", OptValueType::Optional);

    // Parse program's command-line `std::env::args()` with the given
    // option specification (`OptSpecs`).
    let parsed = specs.getopt();

    // With this you can see the parsed Args struct.
    eprintln!("{:#?}", parsed);

    // Report user about unknown options.
    for u in &parsed.unknown {
        eprintln!("Unknown option: {}", u);
    }

    // Report user about missing values for options that require them
    // (i.e. "file"). Exit the program with error code.
    for o in &parsed.required_value_missing() {
        eprintln!("Value is required for option '{}'.", o.name);
        return ExitCode::FAILURE;
    }

    // Print help and exit because "-h" or "--help" was given.
    match parsed.options_first("help") {
        None => (),
        Some(_) => {
            println!("Print friendly help about program's usage.");
            return ExitCode::from(2);
        },
    }

    // Collect all (required) values for "-f" and "--file". Also collect
    // all (optional) values for "-v" and "--verbose". These can be
    // empty vectors if no values were given.
    for f in &parsed.options_value_all("file") {
        println!("File name: {:?}", f);
    }
    for v in &parsed.options_value_all("verbose") {
        println!("Verbose level: {:?}", v);
    }

    // Notice if "-v" or "--verbose" was given (even without a value).
    if parsed.options_first("verbose").is_some() {
        println!("Option 'verbose' was given.");
    }

    // Collect all other arguments.
    for o in &parsed.other {
        println!("Other argument: {:?}", o);
    }
    
    ExitCode::SUCCESS
}
