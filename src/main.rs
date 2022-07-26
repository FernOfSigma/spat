// Copyright (c) 2022 FernOfSigma.
// This software is licensed under the MIT license.

use std::path::PathBuf;

use clap::{arg, command, value_parser};

use spat::shorten;

fn main() {
    let matches = command!()
        .arg(arg!(<DIR> "Path to a directory")
        .value_parser(value_parser!(PathBuf)))
        .get_matches();

    if let Some(path) = matches.get_one::<PathBuf>("DIR") {
        println!("{}", shorten(path).display());
    }
}
