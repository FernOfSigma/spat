// Copyright (c) 2022 FernOfSigma.
// This software is licensed under the MIT license.

//! Shortens UNIX-like paths.

use std::path::{Path, PathBuf};

/// This accepts a [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html)
/// and shortens each of its components to their first chatacters, only leaving
/// the last component unmodified.
///
/// Dots (`.`) are left unchanged as they may refer to hidden or relative paths.
///
/// # Example
/// ```rust
/// use std::path::Path;
///
/// let my_path = Path::new("/home/sigma/.cargo/bin");
/// let shorter = spat::shorten(my_path);
/// println!("{}", shorter.display());
/// // Output: /h/s/.c/bin
/// ```
pub fn shorten(path: &Path) -> PathBuf {
    let mut components = path.components().collect::<Vec<_>>();
    let basename = components.pop();

    let mut shortened = PathBuf::new();

    for component in components.into_iter() {
        let mut s = String::new();
        // Iterate until a character other than '.' is found. This accounts for
        // hidden, relative, and rare directories such as '....' made by sickos.
        for ch in component.as_os_str().to_string_lossy().chars() {
            s.push(ch);
            if ch == '.' {
                continue;
            }
            break;
        }
        shortened.push(s);
    }

    shortened.extend(basename);
    shortened
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(s1: &str, s2: &str) {
        assert_eq!(shorten(Path::new(s1)).to_string_lossy(), s2);
    }

    #[test]
    fn basic() {
        compare("~", "~");
        compare("/", "/");
        compare(".", ".");
    }

    #[test]
    fn normal() {
        compare("/home/sigma/dev/rust/", "/h/s/d/rust");
    }

    #[test]
    fn hidden() {
        compare("~/.cargo/bin", "~/.c/bin");
    }

    #[test]
    fn relative() {
        compare("./files/music/../video", "./f/m/../video");
    }
}
