// https://crates.io/crates/image
// https://github.com/denilsonsa/small_scripts/blob/master/extract_embedded_images_from_svg.py
// https://stackoverflow.com/questions/76174841/how-to-set-the-dpi-of-an-image-before-saving-it
// https://superuser.com/questions/299977/how-to-extract-an-embedded-image-from-a-svg-file
// https://qiita.com/benki/items/6dfd082ab3b03dc6b069
// https://fasterthanli.me/series/dont-shell-out/part-7

use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;

pub mod args;
pub mod err;

pub mod id;
pub mod img;
pub mod inks;

use crate::inks::*;

const SMASHED_DIR: &str = "compressed";

fn main() {
    let args = args::parse();
    let svg = &args.file;

    if !svg.ends_with(".svg") {
        err_exit!(
            "command line",
            "please enter a Inkscape file with .svg extension"
        );
    }

    // format names
    let svg_out = &format_filename(svg);
    let svg_out_tmp = &format!("{}.TMP", svg_out);

    // file create
    let mut out =
        File::create(svg_out_tmp).unwrap_or_else(|err| err_exit!("file output create", err));

    // file open Buffer ro read big files
    let file = File::open(svg).unwrap_or_else(|err| err_exit!("file open", err));
    let reader = io::BufReader::new(file);

    // Load the file contents into memory
    let contents: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap_or_else(|err| err_exit!("file reader", err));

    // Process the lines in parallel, without order
    let processed_lines: Vec<(usize, String)> = contents
        .into_iter()
        .enumerate()
        .par_bridge()
        .map(|(i, line)| process_line(&args, i, &line))
        .collect();

    // Sort the processed lines back into the original order
    let mut sorted_lines: Vec<String> = vec![String::new(); processed_lines.len()];
    for (index, line) in processed_lines {
        sorted_lines[index] = line;
    }

    let inkscape_doc = sorted_lines.join("\n");

    // Write the processed content to a new file
    out.write_all(inkscape_doc.as_bytes())
        .unwrap_or_else(|err| err_exit!("file output write", err));

    // rename tmp to min.svg file
    fs::rename(svg_out_tmp, svg_out).unwrap_or_else(|err| err_exit!("rename TMP file", err));

    // create smashed dir and move compressed file
    if args.move_completed {
        // create_dir_all do not err/pani if path already exists
        fs::create_dir_all(SMASHED_DIR)
            .unwrap_or_else(|err| err_exit!(format!("create {} path", SMASHED_DIR), err));

        // move is done with rename
        fs::rename(svg, format!("{SMASHED_DIR}/{svg}"))
            .unwrap_or_else(|err| err_exit!("rename TMP file", err));
    }
}

pub fn str_mb_size(s: &str) -> String {
    let s_size = s.len();

    // Convert size from bytes to megabytes
    let mb_size = s_size as f64 / (1024.0 * 1024.0);

    // format
    format!("{:.3}MB", mb_size)
}

fn format_filename(filename: &str) -> String {
    // Define the regex patterns for the three cases
    let patterns = vec![
        // important put more extended regex for first in this list
        //r"\.q(100|\d{1,2})\.min\.svg$", // matches .q[1-100].min.svg at the end of the string
        r"\.min\.svg$", // matches .min.svg at the end of the string
        r"\.svg$",      // matches .svg at the end of the string
    ];

    let mut clean_name = filename.to_string();

    // Apply each pattern to remove the matching suffix
    for pattern in patterns {
        let re = Regex::new(pattern).unwrap();
        clean_name = re.replace(&clean_name, "").to_string();
    }

    format!("{}.min.svg", clean_name)
}

//=====================================================================
// UNIT TESTS
//=====================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_format_filename() {
        let tests = vec![
            //     have   ,         want
            ("example.svg", "example.min.svg"),
            ("example.min.svg", "example.min.svg"),
            ("example.q1.min.svg", "example.q1.min.svg"),
            ("example.q100.min.svg", "example.q100.min.svg"),
            ("example.q101.min.svg", "example.q101.min.svg"), // this will not match q101, and it will not be removed
        ];

        for tt in tests {
            let have = format_filename(tt.0);
            let want = tt.1;
            assert_eq!(have, want);
        }
    }
}
