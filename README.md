# dog

dog - a cat alternative written in rust

dog is a replacement for the 'cat' command

dogs features include

- Print line numbers
- Print $ at EOL
- Print statistics (count of lines, characters and uni-characters) at the end of the File
- Print a specified range of lines

## Usage

### Manual installation

#### Linux

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

in zshrc or bashrc
```bash
alias cat='dog'
```

## Third-Party software
### Thanks to all developers
https://github.com/mackwic/colored
https://github.com/TeXitoi/structopt
https://docs.rs/config_struct/0.5.0/config_struct/
https://docs.rs/toml/0.5.8/toml/
https://github.com/chyh1990/yaml-rust
https://github.com/serde-rs/serde
https://github.com/dtolnay/serde-yaml
https://github.com/eminence/terminal-size
https://github.com/freaky/rust-filesize
https://github.com/mackwic/colored