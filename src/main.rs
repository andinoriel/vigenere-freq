extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue};

mod data;
mod util;

use data::DataSet;
use util::Util;

struct Options {
    file: String,
    key: String,

    seq_start: usize,
    seq_end: usize,
    div_start: usize,
    div_end: usize,

    trim_count: usize,
    crop_count: usize,

    verbose: bool,
}

fn main() {
    let mut options = Options {
        file: "".to_string(),
        key: "".to_string(),

        seq_start: 0,
        seq_end: 0,
        div_start: 0,
        div_end: 0,

        trim_count: 0,
        crop_count: usize::MAX,

        verbose: false,
    };

    // this block limits scope of borrows by parser.refer() method
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Frequency analysis of the Vigenere cipher");

        parser
            .refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Detailed output")
            .metavar("V");
        parser
            .refer(&mut options.file)
            .add_option(&["-f", "--file"], Store, "File with a text to cipher")
            .required()
            .metavar("F");
        parser
            .refer(&mut options.key)
            .add_option(&["-k", "--key"], Store, "Key for a text to cipher")
            .required()
            .metavar("K");
        parser
            .refer(&mut options.seq_start)
            .add_option(
                &["--seq-start"],
                Store,
                "Start value of lengths of chars sequences",
            )
            .required()
            .metavar("SS");
        parser
            .refer(&mut options.seq_end)
            .add_option(
                &["--seq-end"],
                Store,
                "End value of lengths of chars sequences",
            )
            .required()
            .metavar("SE");
        parser
            .refer(&mut options.div_start)
            .add_option(
                &["--div-start"],
                Store,
                "Start value of the divisor interval",
            )
            .required()
            .metavar("DS");
        parser
            .refer(&mut options.div_end)
            .add_option(&["--div-end"], Store, "End value of the divisor interval")
            .required()
            .metavar("DE");

        parser
            .refer(&mut options.trim_count)
            .add_option(
                &["--trim-count"],
                Store,
                "Leave only those data that char sequence occur more than specified value",
            )
            .metavar("TC");
        parser
            .refer(&mut options.crop_count)
            .add_option(
                &["--crop-count"],
                Store,
                "Value that indicating how much data to leave for the resulting sample",
            )
            .metavar("CC");

        parser.parse_args_or_exit();
    }

    let text = Util::load_file(&options.file);
    let encrypted = Util::vigenere_cipher(&text, &options.key);

    for i in options.seq_start..options.seq_end + 1 {
        println!("{}':\n", i);

        let mut ds = DataSet::new();

        ds.analyze(&encrypted, i);
        ds.sort();

        ds.trim(options.trim_count);
        ds.crop(options.crop_count);

        if options.verbose {
            println!("{}", ds);
        }

        println!("| {0: ^5} | {1: ^10} | {2: ^6} |", "div", "count", "");
        for (i, j, k) in ds.get_divide_data(options.div_start..options.div_end + 1) {
            println!(
                "| {0: ^5} | {1: ^10} | {2: ^5}% |",
                i,
                j,
                (j as f32 / k as f32 * 100.).round()
            );
        }
        println!("");
    }
}
