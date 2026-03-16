use clap::Parser;
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::process;

mod renderer;
mod theme;

#[derive(Clone, Copy, Default, clap::ValueEnum)]
enum ColorMode {
    Always,
    #[default]
    Auto,
    Never,
}

#[derive(Parser)]
#[command(name = "mdcat", about = "Render markdown files to the terminal")]
struct Cli {
    /// Markdown files to render
    files: Vec<PathBuf>,

    /// Disable color output
    #[arg(long)]
    no_color: bool,

    /// When to use color: always, auto, or never [default: auto]
    #[arg(long, value_enum, default_value_t = ColorMode::Auto)]
    color: ColorMode,
}

fn main() {
    let cli = Cli::parse();

    let color = match cli.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => {
            !cli.no_color
                && std::env::var_os("NO_COLOR").is_none()
                && io::stdout().is_terminal()
        }
    };

    if cli.files.is_empty() {
        eprintln!("mdcat: no input files specified");
        process::exit(1);
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut had_error = false;
    for (i, path) in cli.files.iter().enumerate() {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                if i > 0 {
                    let sep = "─".repeat(72);
                    let _ = writeln!(out, "\n{}\n", sep);
                }
                let rendered = renderer::render(&content, color);
                let _ = out.write_all(rendered.as_bytes());
            }
            Err(e) => {
                eprintln!("mdcat: {}: {}", path.display(), e);
                had_error = true;
            }
        }
    }

    if had_error {
        process::exit(1);
    }
}
