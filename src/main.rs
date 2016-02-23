extern crate term;
extern crate hoedown;

mod term_renderer;
use term_renderer::TermRenderer;
use hoedown::{Markdown, Render};

fn main() {
    use std::process::Command;

    Command::new("reset").spawn().unwrap().wait().unwrap();

    let file = std::fs::File::open("intro.md").unwrap();
    let markdown = Markdown::read_from(file);
    let mut term = TermRenderer::new(term::stdout().unwrap());
    term.render(&markdown);
}
