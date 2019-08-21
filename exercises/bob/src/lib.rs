pub fn reply(message: &str) -> &str {
    let mut has_lower_chars = false;
    let mut has_upper_chars = false;
    let message = message.trim();
    let is_question = message.ends_with('?');

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    message.chars().for_each(|c| match c {
        'a'..='z' => has_lower_chars = true,
        'A'..='Z' => has_upper_chars = true,
        _ => {}
    });

    let is_shouted = has_upper_chars && !has_lower_chars; // 1234 5 A@ $#$##
    match (is_question, is_shouted) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
