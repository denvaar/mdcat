# mdcat

Render Markdown files in your terminal with styled output.

## Features

| Markdown Element | Terminal Rendering |
|-----------------|-------------------|
| **Bold** | Bold text |
| *Italic* | Italic text |
| `inline code` | Colored |
| Code blocks | Colored, indented block |
| Headings | Bold, sized by level |
| Blockquotes | Indented with prefix |
| Lists | Bulleted / numbered |
| Tables | Aligned columns |
| Horizontal rules | Line separator |
| Links | URL displayed inline |

## Installation

Requires [Rust](https://rustup.rs/).

```sh
cargo build --release
```

The binary is placed at `target/release/mdcat`.

Optionally, copy it somewhere on your `$PATH`:

```sh
cp target/release/mdcat ~/.local/bin/
```

## Usage

```
mdcat [OPTIONS] <FILES>...
```

Render one or more Markdown files to the terminal. Multiple files are separated by a blank line in the output.

```sh
mdcat README.md
mdcat doc1.md doc2.md doc3.md
mdcat --no-color README.md
```

Pipe-friendly — color is automatically disabled when stdout is not a TTY:

```sh
mdcat README.md | less
```

## Options

| Option | Description |
|--------|-------------|
| `--no-color` | Disable colored/styled output |
| `--help` | Print help information |

### Environment Variables

| Variable | Description |
|----------|-------------|
| `NO_COLOR=1` | Disable color output (follows the [no-color.org](https://no-color.org) standard) |

Color is also automatically disabled when stdout is not a TTY (e.g., when piping output).

## Examples

```sh
# Render a single file
mdcat README.md

# Render multiple files
mdcat intro.md usage.md faq.md

# Disable color (useful for terminals without color support)
mdcat --no-color README.md

# Disable color via environment variable
NO_COLOR=1 mdcat README.md

# Pipe to a pager (color auto-disabled)
mdcat README.md | less
```
