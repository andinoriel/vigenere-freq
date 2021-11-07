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
./vigenere-freq [OPTIONS]

Frequency analysis of the Vigenere cipher

Optional arguments:
  -h,--help             Show this help message and exit
  -v,--verbose          Detailed output
  -f,--file F           File with a text to cipher
  -k,--key K            Key for a text to cipher
  --seq-start SS        Start value of lengths of chars sequences
  --seq-end SE          End value of lengths of chars sequences
  --div-start DS        Start value of the divisor interval
  --div-end DE          End value of the divisor interval
  --trim-count TC       Leave only those data that char sequence occur more
                        than specified value
  --crop-count CC       Value that indicating how much data to leave for the
                        resulting sample
```

## License

This project is licensed under the [MIT License](LICENSE).

## Credits

My thanks to [nikohonu](https://github.com/nikohonu) for the application idea.
