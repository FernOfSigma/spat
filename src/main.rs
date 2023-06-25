// Copyright (c) 2022 FernOfSigma.
// This software is licensed under the MIT license.

use std::process::exit;

const HELP: &str = "\
USAGE: spat [OPTIONS] <DIR>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information";

const USAGE: &str = "\
error: missing required argument <DIR>
try -h or --help for more information";

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_arg() -> String {
    let mut args = pico_args::Arguments::from_env();

    // Check for mutually exclusive options and exit early.
    if args.contains(["-h", "--help"]) {
        println!("{HELP}");
        exit(0);
    }
    if args.contains(["-V", "--version"]) {
        println!("spat {VERSION}");
        exit(0);
    }

    // Show usage and exit if no argument is given.
    let Ok(arg) = args.free_from_str::<String>() else {
        eprintln!("{USAGE}");
        exit(1);
    };

    // Sanity check for all args being parsed.
    let unused_args = args.finish();
    if !unused_args.is_empty() {
        panic!("unused argument(s): {unused_args:?}");
    }

    arg
}

fn main() {
    println!("{}", spat::shorten(parse_arg()).display());
}

