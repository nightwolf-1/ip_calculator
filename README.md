# IP Calculator

[![forthebadge](./assets/svg/made-with-rust.svg)](https://www.rust-lang.org/)
[![forthebadge](./assets/svg/use-asciinema.svg)](https://asciinema.org/)
[![forthebadge](./assets/svg/use-forthebadge.svg)](https://forthebadge.com)

The IP Calculator is a command-line tool written in Rust designed to calculate and manipulate IP addresses, subnets, and ranges. This utility provides various operations such as validating IP addresses, masks, and CIDR notations, calculating subnets, and finding ranges of addresses.

# Features

Validate if an IP address, mask, or CIDR is correctly formatted.

Perform subnet calculations based on a given CIDR and prefix.

Retrieve a specific subnet by index.

Check if two IP addresses belong to the same subnet.

Find ranges of IP addresses within a given CIDR.

Display help and command usage information.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest statble version)

## Clone and build 
1. Clone the repository
```bash
git clone 
cd ip_calculator
```
2. Build the program
```bash
cargo build --release
```
3. The binary will be avialable in the target/release directory
```bash
./target/release/ip_calculator
```

# Usage

The program accepts a variety of commands, each designed for a specific task. Below is a list of supported commands and their usage:

