pub fn rev_str(input: &str) -> String {
    let mut reverse = String::new();

    reverse = input.chars().rev().collect();
    reverse
}
