# Self Copy Utility (Rust)

A small Rust command-line utility that creates a unique directory and copies its own executable into that location.

This project demonstrates:

- Rust filesystem operations
- Command-line argument parsing
- Path manipulation
- Environment inspection
- Error handling with `Result`
- Working with executable paths

## Features

- Automatically creates a `sandbox` directory
- Generates unique timestamp/process-based directory names
- Copies the running executable into the generated directory
- Supports custom destination paths
- Supports custom output filenames
- Optional verbose logging

## Usage

Run with default settings:

```bash
mycopy.exe