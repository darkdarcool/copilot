use syntect::{self, highlighting::Style};

pub fn highlight_line(text: &String) -> Vec<(Style, &str)> {
    // using syntect, apply markdown syntax highlighting to the text
    let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_extension("md").unwrap();
    let h = syntect::highlighting::ThemeSet::load_defaults();
    let mut highlighter = syntect::easy::HighlightLines::new(syntax, &h.themes["base16-mocha.dark"]);

    let highlighted = highlighter.highlight_line(text, &syntax_set).unwrap();
    // let escaped = syntect::util::as_24_bit_terminal_escaped(&highlighted, false);
    highlighted
}

pub fn to_terminal_escaped(highlighted: &Vec<(Style, &str)>) -> String {
    // convert the highlighted text to a string with terminal escape sequences
    let escaped = syntect::util::as_24_bit_terminal_escaped(highlighted, false);
    escaped
}
