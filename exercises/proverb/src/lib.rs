pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if list.is_empty() {
        return proverb;
    }

    proverb = list[0..list.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, word)| format!("For want of a {} the {} was lost.\n", word, list[i + 1]))
        .collect();

    format!("{}And all for the want of a {}.", proverb, list[0])
}
