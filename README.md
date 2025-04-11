# write_mode [![Latest Version]][crates.io] [![License]][license] [![Documentation]][docs.rs] [![CI]][actions] [![Dependencies]][deps]
<!-- [![Coverage]][codecov] -->

[Latest Version]: https://img.shields.io/crates/v/write_mode.svg
[crates.io]: https://crates.io/crates/write_mode
[License]: https://img.shields.io/crates/l/write_mode.svg
[license]: https://github.com/Siiir/write_mode/blob/main/LICENSE
[Documentation]: https://docs.rs/write_mode/badge.svg
[docs.rs]: https://docs.rs/write_mode
[CI]: https://github.com/Siiir/write_mode/actions/workflows/check.yaml/badge.svg?branch=main
[actions]: https://github.com/Siiir/write_mode/actions/workflows/check.yaml
[Dependencies]: https://deps.rs/repo/github/Siiir/write_mode/status.svg
[deps]: https://deps.rs/repo/github/Siiir/write_mode
[Coverage]: https://codecov.io/gh/Siiir/write_mode/branch/main/graph/badge.svg
[codecov]: https://codecov.io/gh/Siiir/write_mode

## 🎯 Overview

An open-source Rust library, which defines file write modes to allow easier development when interacting with the file system.

Supports both `fs_err` and `std::fs`.

## Examples
```rust
use std::io::Write;
use write_mode::WriteMode;

let user_input = "o"; // Overwrite existing object (file content). Fail if doesn't exist.
let mode: WriteMode = user_input.parse().unwrap(); // Parses many formats & shorthands.
let mut file = mode.std_open("/dev/null").unwrap(); // Some existing file.
file.write_all(b"Some new content for the file.").unwrap();
```

## 📋 Prerequisites

- 📦 `cargo` including `fmt` subcommand

## 🔄 Continuous Integration

This project uses GitHub Actions for continuous integration. The CI pipeline includes:

- ✅ Running all tests (`cargo test --all-targets`)
- 📚 Documentation checks (`cargo rustdoc`)
- 🤖 Automatic formatting fixes with `cargo fmt`
- 💅 Code formatting check `cargo fmt --check`
- ⏪ Automatic reversion of commits that fail CI checks

### 🧹 Code Style Guide (rustfmt)

This project uses a custom [rustfmt](https://github.com/rust-lang/rustfmt) configuration to enforce consistent code formatting. Below are the key formatting rules we are following:

#### 🛠️ Formatting Rules

| Setting                          | Value       | Description                                        |
| -------------------------------- | ----------- | -------------------------------------------------- |
| `max_width`                      | `100`       | Maximum width of a line before breaking            |
| `hard_tabs`                      | `false`     | Use spaces instead of tabs                         |
| `tab_spaces`                     | `4`         | Number of spaces per indentation level             |
| `newline_style`                  | `"Auto"`    | Use system-native line endings (LF/CRLF)           |
| `use_small_heuristics`           | `"Default"` | Enables formatting heuristics for small constructs |
| `fn_call_width`                  | `60`        | Max width for function calls on a single line      |
| `attr_fn_like_width`             | `70`        | Max width for attribute-like function macros       |
| `struct_lit_width`               | `18`        | Max width for struct literals                      |
| `struct_variant_width`           | `35`        | Max width for struct variants in enums             |
| `array_width`                    | `60`        | Max width for arrays on a single line              |
| `chain_width`                    | `60`        | Max width for method chains on a single line       |
| `single_line_if_else_max_width`  | `50`        | Max width for single-line `if/else` expressions    |
| `single_line_let_else_max_width` | `50`        | Max width for single-line `let...else` expressions |
| `reorder_imports`                | `true`      | Automatically sort `use` statements                |
| `reorder_modules`                | `true`      | Automatically sort module declarations             |
| `fn_params_layout`               | `"Tall"`    | Format function params vertically (one per line)   |
| `edition`                        | `2024`      | Rust edition for parsing and formatting            |
| `style_edition`                  | `2024`      | Edition used for formatting style rules            |

### 🧪 Formatting Check in CI

Formatting is checked in CI using:

```bash
cargo fmt --check
```

Run `cargo fmt` before push to automatically format your code.
**The CI workflow will fail if your code does not follow the formatting rules.**

### Reset in the CI

If the workflow fails, the branch will be reset to state before this commit using `git reset --hard HEAD~1` <br />
**How to work after this on the local environment?**

- If you want to discard all changes from the failing commit, run `git pull --rebase`
- If you want to introduce some fixes, just create another commit and push it to try one more time

## 🤝 Contributing Guideline

1. 🍴 **`git clone` the repository**
2. 🌿 **Create new branch from main** (`git checkout -b <branch-name>`)
3. ✏️ **Make your changes** (`git add .; git commit -m <msg>`)  
   **Remember** about subissue closing. ("\<msg\>. Closes #K")
4. 🧪 **Run the tests** (`cargo test`)
5. 💅 **Ensure your code is formatted** (`cargo fmt`)
6. 💾 **Commit your changes** (`git commit -m 'Add some amazing feature'`)
7. 📤 **Push to the branch** (`git push origin <branch-name>`)
8. 🔄 **Open a Merge/Pull Request**  
   **Remember** about closing the MR's target issue "\<description\>.\n Closes #N"

### 🎯 Issue Closing

When working on issues, follow these guidelines:

- **Merge/Pull Requests** should target and close the main issue they're addressing:
- **Individual commits** should close sub-issues or tasks:
