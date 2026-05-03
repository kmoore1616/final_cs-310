# Overview

This is a simple implementation of a archiving system called MTAR, created for a final project for CS-310 - Programming Projects in Rust.

Its use is to zip files together and compress them for ease of transfer and organization. It is compatible with all files with the notable exception of directories.

# Installation and Setup

The perfered way to install the utility is to build from source. Make sure the rust toolchain is installed, and run:

```bash
cargo build --release
```

The binary will be located in:

```text
target/release/final_project
```

If you already have mtar installed, you can download the latest release, and extract the archive using the utility.

# Usage

There are two main ways to use the utility: Archiving and Extracting.

To archive a file run:

```bash
./mtar ARCHIVE_NAME.mtar -a ARCHIVE ARCHIVE...
```

To Extract run:

```bash
./mtar -x ARCHIVE_NAME.mtar
```

## Usage notes

When running the tool, make sure to use the tool within the same directory as the file being processed. Otherwise you will need to recreate the file hierarchy within the extraction location for the tool to work.

# Licences

This project uses flate2, which is licensed under MIT OR Apache-2.0.
