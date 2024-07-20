# PSP WASM

[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen?style=flat-square)](/LICENSE)

It works on PPSSPP, but it doesn't work on the actual hardware.


## Requirements

- [tinygo](https://tinygo.org/)
- [rust-psp](https://github.com/overdrivenpotato/rust-psp)

## How to build

```sh
cd go
tinygo build -o hello.wasm -target=wasi main.go
cd ..
cargo psp
```

now you can use the EBOOT.PBP
