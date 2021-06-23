# dog

dog - a cat alternative written in rust

dog is a replacement for the 'cat' command

dogs features include
- Print line numbers
- Print $ at EOL
- Print statistics (count of lines, characters and uni-characters, size of file) at the end of the file
- Print a specified range of lines

## Usage
    dog [FLAGS] [OPTIONS] <path>

    FLAGS:\
        -h, --help            Prints help information\
        -n, --line_numbers    Print line numbers\
        -q, --quiet           Do not print the file contents\
        -e, --show-ends       Print $ at EOL\
        -s, --statistics      Print statistics (count of lines, characters and uni-characters, actual size of file) at the end of the file\
            --size            print apparent sizes, rather than disk usage\
        -V, --version         Prints version information\

    OPTIONS:\
        -l, --lines <n:m>    Print a specified range of lines [default: :]\

    ARGS:\
        <path>    Give me a file\

## Installation


### Manual installation

install rustc and cargo

```bash
cd /tmp

git clone https://github.com/macmoritz/dog.git
cd /tmp/dog

cargo build --release

cd target/release

./dog
```

To install dog locally
```bash
cd /tmp/dog
cargo install --path .
```
and add this line to your .bashrc or .zshrc

```bash
export PATH=$PATH:$HOME/.cargo/bin
```
### Using dog instead of cat

in .zshrc or .bashrc
```bash
alias cat='dog'
```

## Third-Party software
#### Thanks to all developers
https://github.com/TeXitoi/structopt\
https://github.com/mistodon/config_struct\
https://github.com/serde-rs/serde\
https://github.com/dtolnay/serde-yaml\
https://github.com/eminence/terminal-size\
https://github.com/freaky/rust-filesize\
https://github.com/mackwic/colored\
