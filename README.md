# asa-wrap
ARK Survival Ascended dedicated server wrapper.

## Purpose
This program starts an ARK Survival Ascended dedicated server 
with parameters read from the `GameUserSettings.ini`.

## Installation
You need to have a [Rust toolchain](https://rust-lang.org/) installed.
```shell
$ git clone https://github.com/conqp/asa-wrap.git
$ cd asa-wrap
$ cargo build --release
```
The built binary can be found under `target\release\asa-wrap{,.exe}`.
