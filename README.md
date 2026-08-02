# Rust Executable Copy Utility

A Rust command-line utility that creates a unique directory, copies the running executable into it, and optionally verifies the copied file using SHA-256 hashing.

This project demonstrates:

- Rust filesystem operations
- Command-line argument parsing
- Path handling with `Path` and `PathBuf`
- Error handling with `Result`
- Environment inspection
- Cryptographic hashing
- File integrity verification

## Features

- Creates a `sandbox` directory automatically
- Generates unique output directories using timestamps and process IDs
- Copies the currently running executable
- Supports custom destination paths
- Supports custom output filenames
- Optional verbose diagnostic output
- SHA-256 verification of copied files

## Usage

Run with default settings:

```bash
mycopy.exe