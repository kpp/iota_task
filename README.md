# Ledger Statistics

## Install rust

https://www.rust-lang.org/learn/get-started

## Run

```bash
cargo run --release -- PATH_TO_DATABASE
```

or build in release and run the executable:

```
cargo build --release
./target/release/iota_task PATH_TO_DATABASE
```

## Example

database.txt:

```
5
1 1 0
1 2 0
2 2 1
3 6 3
3 3 2
```

```bash
$ cargo run -- database.txt
AVG DAG DEPTH: 1.3333334
AVG TXS PER DEPTH: 2.5
AVG REF: 1.6666666
TIPS: 2
```

## Test

```bash
cargo test
```


## Measure coverage

Ubuntu deps:

```bash
sudo apt install libcurl4-openssl-dev libelf-dev libdw-dev binutils-dev cmake
```

Install cargo-travis: `cargo install cargo-travis`

Collect the data:

```bash
cargo build # You have to build the project first
cargo coverage
# the report will be in target/kcov/index.html
```
