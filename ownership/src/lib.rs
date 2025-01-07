pub fn first_subword(mut s: String) -> String {
    let mut my_string = String::new();

    for i in s.chars() {
        if i.is_uppercase() && !my_string.is_empty() {
            break;
        }
        if i == '_' { // we don't use i.find('_') because it's not a string but a simple car in i
            break;
        }
        my_string.push(i);
    }
    my_string
}
