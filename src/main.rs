extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue};

mod data;
mod util;

use data::DataSet;
use util::Util;

struct Options {
    filename: String,
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
        filename: "".to_string(),
        key: "".to_string(),

        seq_start: 0,
        seq_end: 0,
        div_start: 0,
        div_end: 0,

        trim_count: 0,
        crop_count: 0,

        verbose: false,
    };

    // this block limits scope of borrows by parser.refer() method
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Frequency analysis of the Vigenere cipher");

        parser.refer(&mut options.filename).add_option(
            &["-f", "--filename"],
            Store,
            "Filename with a text to cipher",
        );
        parser.refer(&mut options.key).add_option(
            &["-k", "--key"],
            Store,
            "Key for a text to cipher",
        );
        parser.refer(&mut options.seq_start).add_option(
            &["--seq-start"],
            Store,
            "Start value of chars strings sequences",
        );
        parser.refer(&mut options.seq_end).add_option(
            &["--seq-end"],
            Store,
            "End value of chars strings sequences",
        );
        parser.refer(&mut options.div_start).add_option(
            &["--div-start"],
            Store,
            "Start value of the divisor interval",
        );
        parser.refer(&mut options.div_end).add_option(
            &["--div-end"],
            Store,
            "End value of the divisor interval",
        );
        parser.refer(&mut options.trim_count).add_option(
            &["--trim-count"],
            Store,
            "Threshold value for found sequences",
        );
        parser.refer(&mut options.crop_count).add_option(
            &["--crop-count"],
            Store,
            "How much data to crop",
        );
        parser.refer(&mut options.verbose).add_option(
            &["-v", "--verbose"],
            StoreTrue,
            "Be more verbose",
        );

        parser.parse_args_or_exit();
    }

    let text = Util::load_file(&options.filename);
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
