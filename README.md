# Advent of Code 2019 solutions

[![Build Status](https://github.com/emlun/adventofcode-2019/workflows/build/badge.svg)](https://github.com/emlun/adventofcode-2019/actions?query=workflow:build)
[![Clippy Status](https://github.com/emlun/adventofcode-2019/workflows/clippy/badge.svg)](https://github.com/emlun/adventofcode-2019/actions?query=workflow:clippy)

To run the solutions:

```
$ cargo run
```

This assumes [Cargo][cargo] is installed, and that the input files are placed at
`inputs/dayXX.in` relative to the current working directory.

To run an individual day, specify the day as a command line argument:

```
$ cargo run 1
```

To run with a different input, specify a file name as a command line argument.
The file name `-` means standard input:

```
$ cargo run 1 foo.txt
$ cargo run 1 - < foo.txt
```

You can also run an Intcode program by specifying `intcode` instead of a day
number. The second argument is a file containing the program; if omitted or set
to `-`, the program is read from standard input. Program input is read from the
first line of standard input in the same format as an Intcode program; when the
program is also read from standard input, the input is instead read from the
second line.

```
$ cargo run intcode add.intcode <<< '4,7'
11

$ cargo run intcode <<< '3,11,3,12,1,11,12,11,4,11,99
4,7'
11
```

Running the benchmarks requires Rust nightly:

```
$ cargo +nightly bench
```


## License

This is [free and unencumbered software released into the public domain][unlicense].


[cargo]: https://doc.rust-lang.org/stable/cargo/
[unlicense]: https://unlicense.org/
