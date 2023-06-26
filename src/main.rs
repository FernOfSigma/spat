// Copyright (c) 2022 FernOfSigma.
// This software is licensed under the MIT license.

use std::process::exit;

const HELP: &str = "\
USAGE: spat [OPTIONS] <DIR>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information";

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "try -h or --help for more information";

fn parse_arg() -> String {
    let mut args = pico_args::Arguments::from_env();

    // Check for mutually exclusive options, exit if found.
    if args.contains(["-h", "--help"]) {
        println!("{HELP}");
        exit(0);
    }
    if args.contains(["-V", "--version"]) {
        println!("spat {VERSION}");
        exit(0);
    }

    // Capture the first argument, exit if none provided.
    let Ok(arg) = args.free_from_str::<String>() else {
        eprintln!("error: missing required argument <DIR>");
        eprintln!("{USAGE}");
        exit(1);
    };

    // Exit if the given argument is an option.
    if arg.starts_with('-') {
        eprintln!("error: option {arg} not recognized");
        eprintln!("{USAGE}");
        exit(1);
    }

    // Sanity check for all arguments being parsed.
    let unused_args = args.finish();
    if !unused_args.is_empty() {
        panic!("unused argument(s): {unused_args:?}");
    }

    arg
}

fn main() {
    println!("{}", spat::shorten(parse_arg()).display());
}
