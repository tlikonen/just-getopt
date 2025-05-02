// In normal Rust development environment you can run this example code
// with commmands like
//     cargo run --example basic -- -abc --foo
//     cargo run --example basic -- -f 123 --file=456 foo -v3 bar

use just_getopt::{OptFlags, OptSpecs, OptValue};

fn main() {
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
        .option("help", "h", OptValue::None) // Arguments: (id, name, value_type)
        .option("help", "help", OptValue::None)
        .option("file", "f", OptValue::RequiredNonEmpty)
        .option("file", "file", OptValue::RequiredNonEmpty)
        .option("verbose", "v", OptValue::OptionalNonEmpty)
        .option("verbose", "verbose", OptValue::OptionalNonEmpty)
        .flag(OptFlags::OptionsEverywhere);

    // Get arguments iterator from operating system and skip the first item
    let args = std::env::args().skip(1); // which is this program's file path.

    // Parse program's command-line with the given specification `specs`.
    let parsed = specs.getopt(args);

    // With this you can see the parsed output which is an `Args`
    // struct.
    eprintln!("{:#?}", parsed);

    // Create a condition variable for possible exit on error.
    let mut error_exit = false;

    // Report user about unknown options.
    for u in &parsed.unknown {
        eprintln!("Unknown option: {}", u);
        error_exit = true;
    }

    // Report user about missing values for options that require them
    // (i.e. "file"). Exit the program with error code.
    for o in parsed.required_value_missing() {
        eprintln!("Value is required for option '{}'.", o.name);
        error_exit = true;
    }

    // Exit if there were bad command-line arguments.
    if error_exit {
        eprintln!("Use '-h' for help.");
        std::process::exit(1);
    }

    // Print help and exit because "-h" or "--help" was given. We use
    // option's identifier string "help" here to find if the correct
    // option was present in the command line. See the `id` argument of
    // `option()` methods above.
    if parsed.option_exists("help") {
        println!("Print friendly help about program's usage.");
        std::process::exit(0);
    }

    // Collect all (required) values for "-f" and "--file". We use
    // option's identifier (id) string "file" to find the option.
    for f in parsed.options_value_all("file") {
        println!("File name: {:?}", f);
    }

    // Notice if "-v" or "--verbose" was given (even without a value).
    // Then collect all its (optional) values. We use option's
    // identifier (id) string "verbose".
    if parsed.option_exists("verbose") {
        println!("Option 'verbose' was given.");
        for v in parsed.options_value_all("verbose") {
            println!("Verbose level: {:?}", v);
        }
    }

    // Collect all other (non-option) arguments.
    for o in &parsed.other {
        println!("Other argument: {:?}", o);
    }
}
