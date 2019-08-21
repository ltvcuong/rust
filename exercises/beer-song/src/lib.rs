pub fn verse(n: i32) -> String {
    let beer_left = bottles_of_beer(n);
    format!("{} on the wall, {}.\n{}, {} on the wall.\n", &beer_left.replace("n", "N"), &beer_left, &action(n), &bottles_of_beer(if n == 0 {99} else {n-1}))
}

fn bottles_of_beer(n: i32) -> String {
    match n {
        0 => String::from("no more bottles of beer"),
        1 => String::from("1 bottle of beer"),
        _ => n.to_string() + " bottles of beer",
    }
}

fn action(n: i32) -> String {
    match n {
        0 => String::from("Go to the store and buy some more"),
        1 => String::from("Take it down and pass it around"),
        _ => String::from("Take one down and pass it around"),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut verses = String::new();
    for i in (end+1..=start).rev() {
        verses.push_str(&verse(i));
        verses.push_str("\n");
    }
    verses.push_str(&verse(end));
    verses
}
