pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val.clone());
    println!("{}", val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    if index < slice.len() {
        &slice[index]
    } else {
        panic!("Index big");
    }
}
