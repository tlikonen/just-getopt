use just_getopt::{OptFlags, OptSpecs, OptValueType};
use std::process::ExitCode;

fn main() -> ExitCode {
    // The `OptSpecs::new()` function below creates a new option
    // specification struct `OptSpecs`. Its `option()` methods configure
    // three different options for logical meanings "help", "file" and
    // "verbose". All three options can be written in command-line with
    // a short variant (like "-h") and a long variant (like "--help").
    // Some options accept or require a value.
    //
    // Flag OptionsEverywhere changes parser's behavior. This flag means
    // that options and other arguments (non-options) can be mixed in
    // their order in the command line. Without the flag the option
    // parsing stops at the first non-option argument and the rest of
    // the command-line is parsed as non-options.
    let specs = OptSpecs::new()
        .flag(OptFlags::OptionsEverywhere) // Argument: (flag)
        .option("help", "h", OptValueType::None) // Arguments: (id, name, value_type)
        .option("help", "help", OptValueType::None)
        .option("file", "f", OptValueType::Required)
        .option("file", "file", OptValueType::Required)
        .option("verbose", "v", OptValueType::Optional)
        .option("verbose", "verbose", OptValueType::Optional);

    // Parse program's command-line `std::env::args()` with the given
    // option specification `specs` which is an `OptSpecs` struct.
    let parsed = specs.getopt();

    // With this you can see the parsed output which is an `Args`
    // struct.
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

    // Print help and exit because "-h" or "--help" was given. We use
    // option's identifier string "help" here to find if the correct
    // option was present in the command line. See the `id` argument of
    // `option()` methods above.
    match parsed.options_first("help") {
        None => (),
        Some(_) => {
            println!("Print friendly help about program's usage.");
            return ExitCode::from(2);
        }
    }

    // Collect all (required) values for "-f" and "--file". Also collect
    // all (optional) values for "-v" and "--verbose". We use options'
    // identifier (id) strings "file" and "verbose" here.
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

    // Collect all other (non-option) arguments.
    for o in &parsed.other {
        println!("Other argument: {:?}", o);
    }

    // Try to run this program with various command-line options to see
    // the output.
    ExitCode::SUCCESS
}
