# Installing from Source

## Ensure Rust Toolchain is Installed

First, make sure that you have the Rust toolchain installed. If you haven’t installed it yet, visit the [official Rust website](https://www.rust-lang.org) and follow the instructions to install Rust and Cargo.

You can check if Rust and Cargo are installed by running:

```bash
rustc --version
cargo --version
```

> You should see version information if they are properly installed.

## Navigate to the Project Directory

Open a terminal and navigate to the root directory of the Katharsis project you [cloned](https://github.com/git-guides/git-clone) from GitHub.

```bash
cd /path/to/your/katharsis/project
```

## Install and build the project.

To install the dependencies specified in the Cargo.toml file, run the following command:

```bash
cargo build --release
```

> - This command not only installs the dependencies but also builds the project.
> - You can run the `cd target/release` command in the root directory of the project to find the katharsis executable file in the release folder.

## Run Tests (Optional)

Run the unit tests to ensure that everything in the project is functioning correctly:

```bash
cargo test
```
