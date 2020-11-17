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