// Copyright (c) 2022 FernOfSigma.
// This software is licensed under the MIT license.

//! Shortens UNIX-like paths.

use std::path::{Path, PathBuf};
use itertools::Itertools;

/// Takes a reference to a [`Path`] and shortens all but its last component to their
/// first characters. Dots (`.`) are left as is since they might refer to hidden or
/// relative paths.
///
/// [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
///
/// # Example
/// ```rust
/// use spat::shorten;
///
/// fn main() {
///     let shortened = shorten("/path/to/something");
///     assert_eq!(shortened.as_os_str(), "/p/t/something");
/// }
/// ```
pub fn shorten(path: impl AsRef<Path>) -> PathBuf{
    let mut components = path.as_ref().components().collect::<Vec<_>>();
    let basename = components.pop();

    let mut shortened = components.into_iter()
        .map(|component| component.as_os_str().to_string_lossy())
        .map(|s| s.chars().take_while_inclusive(|c| c == &'.').collect::<String>())
        .collect::<PathBuf>();

    shortened.extend(basename);
    shortened
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(s1: &str, s2: &str) {
        assert_eq!(shorten(s1).as_os_str(), s2);
    }

    #[test]
    fn symbols() {
        compare("~", "~");
        compare("/", "/");
        compare(".", ".");
    }

    #[test]
    fn standard() {
        compare("~/sigma/dev/rust/", "~/s/d/rust");
        compare("/home/sigma/dev/rust/", "/h/s/d/rust");
        compare("home/sigma/dev/rust/", "h/s/d/rust");
    }

    #[test]
    fn hidden() {
        compare("~/.cargo/bin/", "~/.c/bin");
    }

    #[test]
    fn dots() {
        compare("./music/../videos/1.mkv", "./m/../v/1.mkv");
    }
}

