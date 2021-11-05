# vigenere-freq

[![Build](https://github.com/andinoriel/vigenere-freq/actions/workflows/build.yml/badge.svg)](https://github.com/andinoriel/vigenere-freq/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Frequency analysis of the Vigenere cipher.

## Build

1. Clone this project and then cd to the project folder;

2. Build normally with:
```
$ cargo build --release
```

## Usage

```
./vigenere-freq [OPTIONS] [--FILENAME] [--KEY] [--SEQ-START] [--SEQ-END] [--DIV-START] [--DIV-END]

Frequency analysis of the Vigenere cipher

Positional arguments:
  --filename            Filename with a text to cipher
  --key                 Key for a text to cipher
  --seq-start           Start value of chars strings sequences
  --seq-end             End value of chars strings sequences
  --div-start           Start value of the divisor interval
  --div-end             End value of the divisor interval

Optional arguments:
  -h,--help             Show this help message and exit
  --trim-count TRIM_COUNT
                        Threshold value for found sequences
  --crop-count CROP_COUNT
                        How much data to crop
  -v,--verbose          Be more verbose
```

## License

This project is licensed under the [MIT License](LICENSE).

## Credits

My thanks to [nikohonu](https://github.com/nikohonu) for the application idea.
