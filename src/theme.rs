use crossterm::style::{Attribute, Color, SetAttribute, SetForegroundColor, ResetColor};
use std::fmt::Write as FmtWrite;

pub struct Theme {
    pub color: bool,
}

impl Theme {
    pub fn new(color: bool) -> Self {
        Theme { color }
    }

    fn ansi(&self, s: String) -> String {
        if self.color { s } else { String::new() }
    }

    pub fn begin_heading(&self, level: u32) -> String {
        if !self.color {
            return String::new();
        }
        let mut s = String::new();
        match level {
            1 => {
                let _ = write!(s, "{}{}{}",
                    SetForegroundColor(Color::Cyan),
                    SetAttribute(Attribute::Bold),
                    SetAttribute(Attribute::Underlined));
            }
            2 => {
                let _ = write!(s, "{}{}",
                    SetForegroundColor(Color::Cyan),
                    SetAttribute(Attribute::Bold));
            }
            _ => {
                let _ = write!(s, "{}{}",
                    SetForegroundColor(Color::Blue),
                    SetAttribute(Attribute::Bold));
            }
        }
        s
    }

    pub fn push_bold(&self) -> String {
        self.ansi(format!("{}", SetAttribute(Attribute::Bold)))
    }

    pub fn push_italic(&self) -> String {
        self.ansi(format!("{}", SetAttribute(Attribute::Italic)))
    }

    pub fn push_blockquote(&self) -> String {
        if !self.color {
            return String::new();
        }
        format!("{}{}",
            SetForegroundColor(Color::DarkGrey),
            SetAttribute(Attribute::Italic))
    }

    pub fn push_link_url(&self) -> String {
        if !self.color {
            return String::new();
        }
        format!("{}{}",
            SetForegroundColor(Color::Blue),
            SetAttribute(Attribute::Underlined))
    }

    pub fn full_reset(&self) -> String {
        if !self.color {
            return String::new();
        }
        format!("{}{}", ResetColor, SetAttribute(Attribute::Reset))
    }

    pub fn format_inline_code(&self, text: &str) -> String {
        if !self.color {
            return text.to_string();
        }
        format!("{}{}{}",
            SetForegroundColor(Color::Green),
            text,
            self.full_reset())
    }

    pub fn format_rule(&self, width: usize) -> String {
        let line = "─".repeat(width);
        if !self.color {
            return line;
        }
        format!("{}{}{}",
            SetForegroundColor(Color::DarkGrey),
            line,
            self.full_reset())
    }

    pub fn format_code_block(&self, _lang: &str, code: &str) -> String {
        if !self.color {
            return code.to_string();
        }
        format!("{}{}{}",
            SetForegroundColor(Color::Green),
            code,
            self.full_reset())
    }
}
