// Copyright (c) 2022 FernOfSigma.
// This software is licensed under the MIT license.

const HELP: &str = "\
USAGE: spat [OPTIONS] <DIR>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information";

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "try -h or --help for more information";

fn parse_arg() -> Result<String, i32> {
    let mut args = pico_args::Arguments::from_env();

    // Check for mutually exclusive options, exit if found.
    if args.contains(["-h", "--help"]) {
        println!("{HELP}");
        return Err(0);
    }
    if args.contains(["-V", "--version"]) {
        println!("spat {VERSION}");
        return Err(0);
    }

    // Capture the first argument, exit if none provided.
    let Ok(arg) = args.free_from_str::<String>() else {
        eprintln!("error: missing required argument <DIR>");
        eprintln!("{USAGE}");
        return Err(1);
    };

    // Exit if the given argument is an option.
    if arg.starts_with('-') {
        eprintln!("error: option {arg} not recognized");
        eprintln!("{USAGE}");
        return Err(1);
    }

    // Exit if any extra arguments are provided.
    let extra_args = args.finish();
    if !extra_args.is_empty() {
        eprintln!("error: receieved extra arguments {extra_args:?}");
        eprintln!("{USAGE}");
        return Err(1);
    }

    Ok(arg)
}

fn main() {
    match parse_arg() {
        Ok(arg) => println!("{}", spat::shorten(arg).display()),
        Err(code) => std::process::exit(code),
    }
}
