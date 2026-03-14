use clap::Parser;
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::process;

mod renderer;
mod theme;

#[derive(Parser)]
#[command(name = "mdcat", about = "Render markdown files to the terminal")]
struct Cli {
    /// Markdown files to render
    files: Vec<PathBuf>,

    /// Disable color output
    #[arg(long)]
    no_color: bool,
}

fn main() {
    let cli = Cli::parse();

    let no_color = std::env::var_os("NO_COLOR").is_some()
        || cli.no_color
        || !io::stdout().is_terminal();

    let color = !no_color;

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
