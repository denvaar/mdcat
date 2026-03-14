use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use crossterm::terminal;
use comfy_table::{Table, Cell};
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum FormatFlag {
    Bold,
    Italic,
    Heading(u32),
    BlockQuote,
    Link(String),
}

struct TableState {
    alignments: Vec<Alignment>,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    in_header: bool,
}

pub struct Renderer {
    theme: Theme,
    format_stack: Vec<FormatFlag>,
    table: Option<TableState>,
    list_stack: Vec<Option<u64>>,
    text_buf: Option<String>,
    code_block_buf: Option<String>,
    code_block_lang: String,
    output: String,
}

impl Renderer {
    pub fn new(color: bool) -> Self {
        Renderer {
            theme: Theme::new(color),
            format_stack: Vec::new(),
            table: None,
            list_stack: Vec::new(),
            text_buf: None,
            code_block_buf: None,
            code_block_lang: String::new(),
            output: String::new(),
        }
    }

    fn emit(&mut self, s: &str) {
        if let Some(buf) = &mut self.text_buf {
            buf.push_str(s);
        } else {
            self.output.push_str(s);
        }
    }

    fn emit_raw(&mut self, s: &str) {
        // Always goes to output, bypasses text_buf redirect
        self.output.push_str(s);
    }

    fn in_table(&self) -> bool {
        self.table.is_some()
    }

    fn reapply_formats(&mut self) {
        if !self.theme.color {
            return;
        }
        let reset = self.theme.full_reset();
        self.output.push_str(&reset);
        let stack = self.format_stack.clone();
        for flag in &stack {
            let s = match flag {
                FormatFlag::Bold => self.theme.push_bold(),
                FormatFlag::Italic => self.theme.push_italic(),
                FormatFlag::Heading(n) => self.theme.begin_heading(*n),
                FormatFlag::BlockQuote => self.theme.push_blockquote(),
                FormatFlag::Link(_) => String::new(),
            };
            self.output.push_str(&s);
        }
    }

    fn list_depth(&self) -> usize {
        self.list_stack.len()
    }

    fn terminal_width() -> usize {
        terminal::size().map(|(w, _)| w as usize).unwrap_or(72)
    }

    pub fn render(content: &str, color: bool) -> String {
        let mut renderer = Renderer::new(color);
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(content, options);
        for event in parser {
            renderer.handle(event);
        }
        renderer.output
    }

    fn handle(&mut self, event: Event) {
        match event {
            Event::Start(tag) => self.on_start(tag),
            Event::End(tag) => self.on_end(tag),
            Event::Text(text) => self.on_text(&text),
            Event::Code(text) => self.on_inline_code(&text),
            Event::SoftBreak => {
                if self.text_buf.is_none() && self.code_block_buf.is_none() {
                    self.emit(" ");
                }
            }
            Event::HardBreak => {
                if self.text_buf.is_none() && self.code_block_buf.is_none() {
                    self.emit("\n");
                }
            }
            Event::Rule => {
                let width = Self::terminal_width();
                let rule = self.theme.format_rule(width);
                self.emit_raw(&format!("{}\n", rule));
            }
            Event::Html(_) | Event::InlineHtml(_) => {}
            _ => {}
        }
    }

    fn on_start(&mut self, tag: Tag) {
        match tag {
            Tag::Heading { level, .. } => {
                let n = heading_level(level);
                self.format_stack.push(FormatFlag::Heading(n));
                let prefix = "#".repeat(n as usize);
                let style = self.theme.begin_heading(n);
                self.emit_raw(&format!("{}{} ", style, prefix));
            }
            Tag::Paragraph => {
                // nothing to emit at start
            }
            Tag::Strong => {
                self.format_stack.push(FormatFlag::Bold);
                if !self.in_table() {
                    let s = self.theme.push_bold();
                    self.emit(&s);
                }
            }
            Tag::Emphasis => {
                self.format_stack.push(FormatFlag::Italic);
                if !self.in_table() {
                    let s = self.theme.push_italic();
                    self.emit(&s);
                }
            }
            Tag::BlockQuote(_) => {
                self.format_stack.push(FormatFlag::BlockQuote);
                let s = self.theme.push_blockquote();
                self.emit_raw(&s);
                self.emit_raw("> ");
            }
            Tag::CodeBlock(kind) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                self.code_block_lang = lang;
                self.code_block_buf = Some(String::new());
            }
            Tag::List(start) => {
                self.list_stack.push(start);
            }
            Tag::Item => {
                let depth = self.list_depth();
                let indent = "  ".repeat(depth.saturating_sub(1));
                let prefix = match self.list_stack.last_mut() {
                    Some(Some(n)) => {
                        let num = *n;
                        *n += 1;
                        format!("{}{}. ", indent, num)
                    }
                    Some(None) => format!("{}• ", indent),
                    None => String::new(),
                };
                self.emit_raw(&prefix);
            }
            Tag::Table(alignments) => {
                self.table = Some(TableState {
                    alignments,
                    header: Vec::new(),
                    rows: Vec::new(),
                    current_row: Vec::new(),
                    in_header: true,
                });
            }
            Tag::TableHead => {
                if let Some(state) = &mut self.table {
                    state.in_header = true;
                }
            }
            Tag::TableRow => {
                if let Some(state) = &mut self.table {
                    state.current_row = Vec::new();
                }
            }
            Tag::TableCell => {
                self.text_buf = Some(String::new());
            }
            Tag::Link { dest_url, .. } => {
                self.format_stack.push(FormatFlag::Link(dest_url.to_string()));
            }
            Tag::Image { .. } => {}
            _ => {}
        }
    }

    fn on_end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Heading(_) => {
                self.format_stack.retain(|f| !matches!(f, FormatFlag::Heading(_)));
                let reset = self.theme.full_reset();
                self.emit_raw(&format!("{}\n", reset));
            }
            TagEnd::Paragraph => {
                self.emit_raw("\n\n");
            }
            TagEnd::Strong => {
                self.format_stack.retain(|f| !matches!(f, FormatFlag::Bold));
                if !self.in_table() {
                    self.reapply_formats();
                }
            }
            TagEnd::Emphasis => {
                self.format_stack.retain(|f| !matches!(f, FormatFlag::Italic));
                if !self.in_table() {
                    self.reapply_formats();
                }
            }
            TagEnd::BlockQuote(_) => {
                self.format_stack.retain(|f| !matches!(f, FormatFlag::BlockQuote));
                let reset = self.theme.full_reset();
                self.emit_raw(&format!("{}\n", reset));
            }
            TagEnd::CodeBlock => {
                let lang = self.code_block_lang.clone();
                if let Some(code) = self.code_block_buf.take() {
                    let formatted = self.theme.format_code_block(&lang, &code);
                    self.emit_raw(&formatted);
                    self.emit_raw("\n");
                }
            }
            TagEnd::List(_) => {
                self.list_stack.pop();
                if self.list_stack.is_empty() {
                    self.emit_raw("\n");
                }
            }
            TagEnd::Item => {
                self.emit_raw("\n");
            }
            TagEnd::TableCell => {
                let cell_text = self.text_buf.take().unwrap_or_default();
                if let Some(state) = &mut self.table {
                    state.current_row.push(cell_text);
                }
            }
            TagEnd::TableHead => {
                if let Some(state) = &mut self.table {
                    state.header = std::mem::take(&mut state.current_row);
                    state.in_header = false;
                }
            }
            TagEnd::TableRow => {
                if let Some(state) = &mut self.table {
                    if !state.in_header {
                        let row = std::mem::take(&mut state.current_row);
                        state.rows.push(row);
                    }
                }
            }
            TagEnd::Table => {
                if let Some(state) = self.table.take() {
                    let rendered = render_table(state);
                    self.emit_raw(&rendered);
                    self.emit_raw("\n");
                }
            }
            TagEnd::Link => {
                let url = self.format_stack.iter().rev().find_map(|f| {
                    if let FormatFlag::Link(u) = f { Some(u.clone()) } else { None }
                });
                self.format_stack.retain(|f| !matches!(f, FormatFlag::Link(_)));
                if let Some(url) = url {
                    let style = self.theme.push_link_url();
                    let reset = self.theme.full_reset();
                    self.emit(&format!(" {}({}){}", style, url, reset));
                    self.reapply_formats();
                }
            }
            TagEnd::Image => {}
            _ => {}
        }
    }

    fn on_text(&mut self, text: &str) {
        if let Some(buf) = &mut self.code_block_buf {
            buf.push_str(text);
        } else {
            self.emit(text);
        }
    }

    fn on_inline_code(&mut self, text: &str) {
        if self.in_table() {
            // In table: emit plain text (ANSI corrupts column widths)
            self.emit(text);
        } else {
            let formatted = self.theme.format_inline_code(text);
            self.emit(&formatted);
        }
    }
}

fn heading_level(level: HeadingLevel) -> u32 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn render_table(state: TableState) -> String {
    let mut table = Table::new();
    table.load_preset("││──├─┼┤│    ┬┴╭╮╰╯");

    for (i, alignment) in state.alignments.iter().enumerate() {
        let comfy_align = match alignment {
            Alignment::Left | Alignment::None => comfy_table::CellAlignment::Left,
            Alignment::Center => comfy_table::CellAlignment::Center,
            Alignment::Right => comfy_table::CellAlignment::Right,
        };
        table.column_mut(i).map(|c| c.set_cell_alignment(comfy_align));
    }

    if !state.header.is_empty() {
        table.set_header(state.header.iter().map(|h| Cell::new(h)));
    }

    for row in &state.rows {
        table.add_row(row.iter().map(|c| Cell::new(c)));
    }

    table.to_string()
}

pub fn render(content: &str, color: bool) -> String {
    Renderer::render(content, color)
}
