# Tail

* Print the last a few bytes `-c` or lines `-n` to stdout
* With multiple files, print header `==> XXX <==` but header can be supressed using the `-q` flag
* When parsing the input for `-c` or `-n`, a positive sign means to print **from** the specified location, while a negative sign (or no sign) means to print the last $N$ bytes/lines
* No need to handle `stdin` for the challenge program
* `-c` and `-n` should be mutually exclusive
* Use a struct `TakeValue` to represent values with a positive and a negative zero
  * Try using `FromStr` so that we can use `parse`
  * Unit test the parser!
  * Use regular expression to capture the number and the sign. Use a `OnceCell` so that the regular expression can be compiled once per program run (it's a bad idea to repeatedly compile regular expression)
* Use the `Seek` trait so that the file cursor can jump to the specified byte and start reading from there
* Use the `time` or the `cargo install hyperfine` binary for performance benchmarking