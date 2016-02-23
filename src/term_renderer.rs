use hoedown::renderer::list::List;
use hoedown::renderer::Table;
use hoedown::{Markdown, Render, Buffer};

use std::io::Write;
use term::StdoutTerminal;
use term::Attr::*;

pub struct TermRenderer {
    term: Box<StdoutTerminal>
}

#[allow(unused_variables)]
impl TermRenderer {
    pub fn new(term: Box<StdoutTerminal>) -> TermRenderer {
        TermRenderer { term: term }
    }
}

impl Render for TermRenderer {
    fn code_block(&mut self, output: &mut Buffer, text: &Buffer, lang: &Buffer) {
        writeln!(self.term, "CODE::: \n{:?} ::: {:?}", text.to_str().unwrap(),
        lang.to_str().unwrap());
    }

    fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING QUOTE_BLOCK HANDLER\n").unwrap();
    }

    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
        self.term.fg(term::color::GREEN).unwrap();
        self.term.attr(Bold).unwrap();
        self.term.attr(Underline(true)).unwrap();

        writeln!(self.term, "{}", content.to_str().unwrap());
        writeln!(self.term, "");
        self.term.reset();
    }

    fn horizontal_rule(&mut self, output: &mut Buffer) {
        output.write(b"MISSING HORIZONTAL_RULE HANDLER\n").unwrap();
    }

    fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: List) {
        writeln!(self.term, "{}", content.to_str().unwrap());
    }

    fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: List) {
        self.term.fg(term::color::WHITE).unwrap();
        write!(self.term, "- {}", content.to_str().unwrap());
        self.term.reset();
    }

    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
        self.term.fg(term::color::WHITE).unwrap();
        let content_string = content.to_str().unwrap();
        let lines : Vec<&str> = content_string.split('\n').collect();
        for line in lines { writeln!(self.term, "{}", line); }
        writeln!(self.term, "\n");
        self.term.reset();
    }

    fn table(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING TABLE HANDLER\n").unwrap();
    }

    fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING TABLE_HEADER HANDLER\n").unwrap();
    }

    fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING TABLE_BODY HANDLER\n").unwrap();
    }

    fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING TABLE_ROW HANDLER\n").unwrap();
    }

    fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: Table) {
        output.write(b"MISSING TABLE_CELL HANDLER\n").unwrap();
    }

    fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write(b"MISSING FOOTNOTES HANDLER\n").unwrap();
    }

    fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, num: u32) {
        output.write(b"MISSING FOOTNOTE_DEFINITION HANDLER\n").unwrap();
    }

    fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {
        output.write(b"MISSING HTML_BLOCK HANDLER\n").unwrap();
    }
}
