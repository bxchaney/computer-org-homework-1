# computer-org-homework-1

## Configuration Info

Operating System: Ubuntu 22.04

Implementation Language: Rust

rustc version: 1.69.0

cargo version: 1.69.0

IDE: Visual Studio Code version 1.75.1 with the most recent version of the Rust extension pack.

Build System: Cargo

I have included my compiled executable in the top directory of this zip as well as in the target/release folder where the build system places it by default.

## Building and Running

The user would have to download and install the Rust development toolchain to build from this source code. More information is available at the Rust website: https://www.rust-lang.org/

Once the development toolchain is installed, the release version of this program can be build by calling `cargo build -r` from the command line when in the top level directory containing this program.

The script `tests.sh` runs each function contained in the program. Please note that this is a bash script and is only commpatible with environments that can run bash scripts.