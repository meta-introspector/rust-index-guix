# index-guix Crate Source Mirror

This repository serves as a mirror for the source code of the `index-guix` Rust crate, as published on [crates.io](https://crates.io/crates/index-guix).

The primary purpose of this repository is to provide a stable Git reference for vendoring or submoduling the `index-guix` crate's source code into other projects, especially when direct access to `crates.io` archives is not feasible or a Git-based workflow is preferred.

## How this repository is populated

The source code in this repository is obtained by downloading the `.crate` archive from `crates.io` and extracting its contents. It is intended to reflect the exact source code of the published crate version.

## Updating the source

To update the source code to a newer version of the `index-guix` crate, use the `update.sh` script located in this repository. This script performs the following steps:
1.  Downloads the specified version of the `index-guix` crate from `crates.io`.
2.  Extracts its contents, replacing the existing source files in this repository.
3.  Cleans up the downloaded `.crate` file.

After running `update.sh`, you should commit and push the changes to this repository to update the mirrored source.

