## Setup
https://www.rust-lang.org/tools/install
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```
~/.rustup => RUSTUP_HOME

~/.cargo => CARGO_HOME

/.cargo/bin => cargo, rustc, rustup

```
rustup self uninstall
```

## cargo
build & package manager
```
cargo new rust-ex
cargo check // only type checking
cargo build // type checking + generate build artiifacts
cargo build --release
cargo run
```
```
./target/debug/rust-ex
```

## cargo single
```
cargo install cargo-single
cargo single run main.rs hello
```


## vscode
```
  "[rust]": {
    "editor.formatOnSave": true
  },
```
alt+1 => move end
cmd+k { => select all content inside curly bracket