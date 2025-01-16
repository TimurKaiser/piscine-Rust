pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut chars = input.chars();
    let first = chars.next().unwrap();
    let rest: String = chars.collect();

    format!("{}{}", first.to_uppercase(), rest)
}

pub fn title_case(input: &str) -> String {
    let words: Vec<String> = input
        .split_whitespace()
        .map(|word| capitalize_first(word))
        .collect();

    words.join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
        } else if c.is_lowercase() {
            result.push(c.to_uppercase().next().unwrap());
        } else {
            result.push(c);
        }
    }

    result
}