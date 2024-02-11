use regex::Regex;

pub type ColoredText = Vec<(String, String)>;

lazy_static! {
    static ref COLOR: Regex = Regex::new(r"<(?P<color>[^>]+)>").unwrap();
}

pub fn get_content_of_colored_text(colored_text: &ColoredText) -> String {
    colored_text
        .iter()
        .map(|(_, text)| text.as_ref())
        .collect::<Vec<&str>>()
        .join("")
}

pub fn get_content_of_raw_colored_text(line: &String) -> String {
    get_content_of_colored_text(&get_colored_text(line))
}

pub fn get_colored_text(line: &String) -> Vec<(String, String)> {
    let mut colored_text = Vec::new();
    let mut seen = 0;
    let mut active_color = None;
    for capture in COLOR.captures_iter(&line) {
        let color = capture.get(1).unwrap();
        let color_start = color.start() - 1;
        if color_start > seen {
            let text = &line[seen..color_start];
            colored_text.push((
                active_color.unwrap_or("white").to_string(),
                text.to_string(),
            ));
        }
        seen = color.end() + 1;
        active_color = Some(color.as_str());
    }
    if seen < line.len() {
        let text = &line[seen..];
        colored_text.push((
            active_color.unwrap_or("white").to_string(),
            text.to_string(),
        ));
    }
    colored_text
}
