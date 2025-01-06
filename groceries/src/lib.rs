pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
    println!("{}", val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    if index < slice.len() {
        &slice[index]
    } 
}
