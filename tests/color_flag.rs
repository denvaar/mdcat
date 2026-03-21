use std::process::Command;

fn mdcat() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mdcat"))
}

#[test]
fn color_always_emits_ansi() {
    let out = mdcat()
        .args(["--color=always", "test.md"])
        .output()
        .expect("failed to run mdcat");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains('\x1b'),
        "--color=always should emit ANSI codes"
    );
}

#[test]
fn color_never_has_no_ansi() {
    let out = mdcat()
        .args(["--color=never", "test.md"])
        .output()
        .expect("failed to run mdcat");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains('\x1b'),
        "--color=never should not emit ANSI codes"
    );
}

#[test]
fn no_color_flag_has_no_ansi() {
    let out = mdcat()
        .args(["--no-color", "test.md"])
        .output()
        .expect("failed to run mdcat");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains('\x1b'),
        "--no-color should not emit ANSI codes"
    );
}

#[test]
fn auto_mode_without_tty_has_no_ansi() {
    // stdout is a pipe in this context, so auto mode should produce no color
    let out = mdcat()
        .arg("test.md")
        .output()
        .expect("failed to run mdcat");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains('\x1b'),
        "auto mode with non-TTY stdout should not emit ANSI codes"
    );
}
