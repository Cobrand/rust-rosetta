// This build script checks that all files in the `src` directory have lines of at most 100
// characters and don't contain trailing whitespace.
//
// It also ensures that comment indicating which task a file solves is at the top.
//
// In case we find a line that doesn't comply with this rules, the build will fail and indicate
// the cause of the problem.
extern crate regex;

use std::fs::{self, File, metadata};
use std::io::Read;
use std::path::Path;

use regex::Regex;

fn main() {
    let files = fs::read_dir("src").unwrap()
                                   .map(|e| e.unwrap());

    for f in files {
        let path = f.path();
        if metadata(&path).unwrap().is_file() {
            check(&path);
        }
    }
}

fn check(path: &Path) {
    let mut content = String::new();
    File::open(&path).unwrap().read_to_string(&mut content).unwrap();

    for (i, mut line) in content.lines().enumerate() {
        if i == 0 && path.file_name().unwrap() != "lib.rs" {
            // Ensure the first line has a URL of the proper form
            let task_comment = Regex::new(r"// http://rosettacode\.org/wiki/.+").unwrap();
            if !task_comment.is_match(line) {
                line_error(i + 1, path, "file does not start with \
                           \"// http://rosettacode.org/wiki/<TASK NAME>\"");
            }
        }

        // Ignore '\r'
        if let Some('\r') = line.chars().rev().next() {
            line = &line[..line.len() - 1];
        }

        // Check length
        if line.len() > 100 {
            line_error(i + 1, path, "line is longer than 100 characters");
        }

        // Check trailing whitespace
        if let Some(last_char) = line.chars().rev().next() {
            if last_char.is_whitespace() {
                line_error(i + 1, path, "line has trailing whitespace");
            }
        }
    }
}

fn line_error(line: usize, path: &Path, msg: &str) {
    panic!("Formatting error, {} (line {} of file \"{}\")",
           msg, line, path.to_str().unwrap())
}
