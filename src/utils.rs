use regex::Regex;

pub fn decolorize(input: &str) -> String {
    let ansi_escape_pattern = r"\u{1b}\[[0-?]*[ -/]*[@-~]";
    let re = Regex::new(ansi_escape_pattern).unwrap();
    re.replace_all(input, "").to_string()
}
