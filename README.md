# sli2dli

SLI to DLI implementation written in rust. Nothing more, nothing less.

## Prerequisites

- [Decent Compiler](http://llvm.org/)
- [Rust - programming language](https://www.rust-lang.org/)
- [Cargo - packages for Rust](https://crates.io/)

## Usage

```
$ sli2dli -h
 0.1.0
Tomas Korcak <korczis@gmail.com>
Disk Usage Information

USAGE:
    sli2dli [FLAGS] [OPTIONS] <FILE>... --manifest <manifest>

FLAGS:
    -c, --cache         Cache results
        --has-header    CSV has header row
    -h, --help          Prints help information
    -V, --version       Prints version information
    -v, --verbose       Verbose mode

OPTIONS:
    -d, --delimiter <delimiter>    Delimiter [default: ,]
    -m, --manifest <manifest>      Path to manifest file

ARGS:
    <FILE>...    Files to process
```

## References

### Articles

- [Bloom Filter](https://en.wikipedia.org/wiki/Bloom_filter) - Space-efficient probabilistic data structure.
- [Cuckoo](https://en.wikipedia.org/wiki/Cuckoo_hashing) - Scheme for resolving hash collisions.

### Libraries

- [crossbeam](https://github.com/crossbeam-rs/crossbeam) - Support for parallelism and low-level concurrency in Rust.
- [csv](https://github.com/BurntSushi/rust-csv) - A CSV parser with type based decoding for Rust.
- [mio](https://github.com/carllerche/mio) - Metal IO library for Rust.
- [mioco](https://github.com/dpc/mioco) - Scalable, coroutine-based, fibers/green-threads for Rust programming language.

### Other

- [Fowler–Noll–Vo hash function](https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function)
- [Benchmarks of different Rust hashing algorithm implementations](http://cglab.ca/~abeinges/blah/hash-rs/)
