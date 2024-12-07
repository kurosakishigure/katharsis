# Contributing to Katharsis

Thank you for contributing to [Katharsis](https://github.com/kurosakishigure/katharsis). The primary way to contribute is through the [GitHub](https://github.com) platform, and we greatly appreciate every contribution you make.

> - Before jumping into a PR be sure to search [existing PRs](https://github.com/kurosakishigure/katharsis/pulls) or [issues](https://github.com/kurosakishigure/katharsis/issues) for an open or closed item that relates to your submission.
> - You need to confirm whether the PR corresponds to an issue. If there isn’t a corresponding issue, please open one before submitting the PR.

## Guidelines

We believe that following the guidelines when contributing to the project helps maintain a consistent style, preventing any confusion during the development process.

### Documentation

Here are some guidelines to maintain a consistent style and voice across the docs:

- Write clear, concise sentences. Avoid tangents.
    - If you find yourself using a lot of commas, consider breaking the sentence into multiple sentences or use a list.
    - Swap out complex words for simpler ones. For example, use instead of utilize.
- Be mindful with the word this. It can be ambiguous and confusing, don't be afraid to repeat the subject of the sentence if unclear.
    - For example, Katharsis uses Rust instead of Katharsis uses this.
- Use an active voice instead of passive. An active sentence is easier to read.
    - For example, Katharsis uses Rust instead of Rust is used by Katharsis. If you find yourself using words like was and by you may be using a passive voice.
- Avoid using words like easy, quick, simple, just, etc. This is subjective and can be discouraging to users.
- Avoid negative words like don't, can't, won't, etc. This can be discouraging to readers.
- Write in second person (you/your). This is more personal and engaging.
- Use gender-neutral language. Use developers, users, or readers, when referring to the audience.
- If adding code examples, ensure they are properly formatted and working.

While these guidelines are not exhaustive, they should help you get started. If you'd like to dive deeper into technical writing, check out the [Google Technical Writing Course](https://developers.google.com/tech-writing/overview).

### Code

When you first build the project, [rustup](https://github.com/rust-lang/rustup) will automatically configure the toolchain for you based on the information in the [rust-toolchain.toml](rust-toolchain.toml) file. For more information about the toolchain, please refer to the relevant official documentation:

- [The toolchain file](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)
- [rustfmt](https://rust-lang.github.io/rustfmt)
- [Clippy](https://doc.rust-lang.org/clippy)
- [rust-analyzer](https://rust-analyzer.github.io)

When writing code, you need to ensure that you have the corresponding documentation and test cases, and maintain a consistent code style. Before performing any actions, such as submitting a PR or opening an issue, follow the relevant templates and process guidelines.

## Code of Conduct

Do note that our [Code of Conduct](CODE_OF_CONDUCT.md) applies to all Katharsis community channels. Users are **highly encouraged** to read and adhere to them to avoid repercussions.

## Developing

- The development branch is `canary`.
- All pull requests should be opened against `canary`.

### Dependencies

- Install Rust and Cargo via [rustup](https://rustup.rs).
- Install the [GitHub CLI](https://github.com/cli/cli#installation).

### Local Development

1. Clone the Katharsis repository (download only recent commits for faster clone):
   ```bash
   gh repo clone kurosakishigure/katharsis -- --filter=blob:none --branch canary --single-branch
   ```
2. Create a new branch:
   ```bash
   git checkout -b MY_BRANCH_NAME origin/canary
   ```
3. Install dependencies and build the project:
   ```bash
   cargo build
   ```
4. Run unit tests:
   ```bash
   cargo test
   ```
5. Start making changes to your code.
   _Note: Don’t forget to run the unit tests once before committing to the branch._
6. When your changes are finished, commit them to the branch:
   ```bash
   git add .
   git commit -m "DESCRIBE_YOUR_CHANGES_HERE"
   ```
7. To open a pull request you can use the GitHub CLI which automatically forks and sets up a remote branch. Follow the prompts when running:
   ```bash
   gh pr create
   ```
