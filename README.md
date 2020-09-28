# Bibterm

Command-line bible verse lookup tool

## Description

Look up KJV scriptures from the command-line

## Usage

`cargo run [book name] [chapter] [verses]`

Book names can either be full (case insensitive) or abbreviations (Gen, Rev, etc.).
Chapters must be numeric.
Verses can either be individual and separated by space (1 5 9) or a range denoted by a hyphen (11-16).

### Acknowledgements

rkazakov, for the KJV file in JSON format [here](https://github.com/rkazakov/usfm2json)
