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

### Dependencies
 - Rust standard library

## Building and Running 
1. Clone the repository
```bash
git clone https://github.com/nightwolf-1/ip_calculator.git
cd ip_calculator
```
2. Build the program
```bash
cargo build --release
```
3. The binary will be avialable in the target/release directory
```bash
./target/release/ip_calculator <command> <arguments>
```

# Usage

The program accepts a variety of commands, each designed for a specific task. Below is a list of supported commands and their usage:

# General Syntax
```bash
./ip_calculator <command> <arguments>
```

# Commands

## Help
- **-h** or **--help**: Displays the list of available commands or detailed help for a specific command.
```bash
./ip_calculator --help
./ip_calculator --help <command>
```

## Subnet Calculations
- **-s** or **--subnets**: Calculate subnets for a given CIDR and prefix. Supports optional filters and pagination.
```bash
./ip_calculator -s <CIDR> <prefix> [-f <number_of_subnets>] [-p <page_number>]
```

## Retrieve Specific Subnet
- **--get-subnet**: Retrieve a specific subnet by index.
```bash
./ip_calculator --get-subnet <CIDR> <prefix> <index>
```

## Check Same Subnet
- **-same** or **--same-subnet**: Check if two IP addresses belong to the same subnet. Optionally supports different masks for the two addresses.
```bash
./ip_calculator -same <IP1> <IP2> <mask1> [mask2]
```

## Validate IP Address
- **-cip** or **--check-ip**: Validate if an IP address is correctly formatted.
```bash
./ip_calculator -cip <IP>
```

## Validate Mask
- **-cmask** or **--check-mask**: Validate if a mask is correctly formatted.
```bash
./ip_calculator -cmask <mask>
```

## Find Address Range
- **-fr** or **--find-range**: Find ranges of addresses within a given CIDR.
```bash
./ip_calculator -fr <CIDR> <number_of_hosts> [excluded_IPs...]
```

## Display CIDR Details
- **CIDR**: Display details about a given CIDR without additional commands.
```bash
./ip_calculator <CIDR>
./ip_calculator <IP> <mask>
```