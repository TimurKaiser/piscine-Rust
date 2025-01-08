pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let c_f64 = c as f64;
    let exp_value = c_f64.exp();
    let log_value = c_f64.abs().ln();

    (c, exp_value, log_value)
}

// Martin give me that, i need to retry to do this func
/// This function takes a string and returns a tuple of two strings.
/// The first string is a clone of the original string.
/// The second string contains the exponential values of the numbers in the string, separated by spaces.
pub fn str_function(a: String) -> (String, String) {
    let original = a.clone();
    let exponantial_values = a
        .split_whitespace()
        //.map|x|: apply a transformation to each substring 'x' in the sequence
        // format!("{}",: create a new string by formatting the result of the following expression
        //x.parse::<f64>().unwrap().exp(): parse the substring 'x' as a 64-bit floating-point number, 
        // unwrap the result (i.e., panic if the parsing fails), 
        // and calculate the exponential of the resulting number
        .map(|x| format!("{}", x.parse::<f64>().unwrap().exp()))
        // collect the results of the map operation into a vector of strings
        .collect::<Vec<String>>()
        .join(" ");

    (original, exponantial_values)
}


//pub fn str_function(a: String) -> (String, String) {
//    let str_to_f64 = a.parse::<f64>();
//
//    let result = if let Ok(n) = str_to_f64 {
//        n.exp().to_string()
//    } else {
//        String::new()  // if err
//    };
//
//    (a, result)
//}



pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let b_f64: Vec<f64> = b.iter()
        .map(|&x| f64::ln(x.abs() as f64))
        .collect();
    
    (b, b_f64)

}
