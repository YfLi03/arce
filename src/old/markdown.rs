/*
using pulldown_cmark to render md files to html String / file
*/

use pulldown_cmark::{html, Options, Parser};
use std::fs::{read_to_string, File};
use std::io::Write;

//this func renders a source *file* to a dst *file*
pub fn render_file(src: &str, dst: &str) {
    let md_str = read_to_string(&src).expect("Cannot read markdown src");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&md_str, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let mut f = File::create(&dst).expect(format!("Could not create file: ").as_str());

    f.write_all(html_output.as_bytes())
        .expect(format!("Could not write to file: ").as_str());

    println!("Markdown Rendered: {} to {}", src, dst);
}

//this func render a source var(str) to a String
pub fn render(md_str: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&md_str, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}
