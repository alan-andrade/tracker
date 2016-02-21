extern crate term;
use std::io::prelude::*;

fn main() {
    //use std::process::{Command, Stdio};
    //let mut child = Command::new("vim").
    //arg("intro.md").
    //stdin(Stdio::inherit()).
    //stdout(Stdio::inherit()).
    //spawn().
    //unwrap();

    //let result = child.wait().unwrap();
    //println!("{:?}", result);

    //use hoedown::Markdown;
    //let markdown = Markdown::new("# Title");

    let mut t = term::stdout().unwrap();

    t.fg(term::color::GREEN).unwrap();
    t.attr(term::Attr::Standout(true)).unwrap();
    write!(t, "hello, ").unwrap();
    writeln!(t, "world!").unwrap();

    t.reset().unwrap();
}
