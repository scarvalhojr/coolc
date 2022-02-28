pub fn escape_str(s: &str) -> String {
    s.chars().fold(String::new(), |mut string, ch| {
        match ch {
            '\\' => string.push_str(r"\\"),
            '\n' => string.push_str(r"\n"),
            '\t' => string.push_str(r"\t"),
            '\u{08}' => string.push_str(r"\b"),
            '\u{0C}' => string.push_str(r"\f"),
            c => string.push(c),
        };
        string
    })
}
