//pub fn delete_and_backspace(s: &mut String) {
//    let mut index = 0;
//    for i in s.chars().iter_mut() {
//        if i == '+' {
//            s.remove(index);
//            index += 1;
//            s.remove(index);
//            continue;
//
//        }
//        index += 1;
//    }
//
//    let mut indexneg = 0;
//    for i in s.chars().iter_mut() {
//        if i == '-' {
//            s.remove(indexneg - 1);
//            indexneg -= 1;
//            s.remove(indexneg);
//            continue;
//        }
//        indexneg += 1;
//    }
//
//}
//

// Louis make it, i take to more time on this func without have the good version


pub fn delete_and_backspace(s: &mut String) {
    let mut rslt = String::new(); 

    let specific_char_minus = '-'; 
    let specific_char_plus = '+'; 
    
    for c in s.chars() {
        //// println!("{}", c);
        rslt.push(c);
        if c == specific_char_minus {
            rslt.pop(); 
            rslt.pop(); 
        }
    }
    //// println!("{}", rslt);
    
    let mut rslt2 = String::new(); 
    let rev_str: String = rslt.chars().rev().collect();
    //// println!("{}", rev_str);

    for r in rev_str.chars() {
        //// println!("{}", r);
        rslt2.push(r);
        if r == specific_char_plus {
            rslt2.pop(); 
            rslt2.pop();
        }
    }

    let final_str: String = rslt2.chars().rev().collect();
    println!("{}", final_str);
    *s = final_str;
}

pub fn do_operations(v: &mut [String]) {
    for expr in v.iter_mut() {
        let mut left = String::new();
        let mut right = String::new();
        let mut operator = ' ';
        let mut found_operator = false;

        for c in expr.chars() {
            if c == '+' || c == '-' {
                operator = c;
                found_operator = true;
                continue;
            }

            if found_operator {
                right.push(c);
            } else {
                left.push(c);
            }
        }

        let left_num: i32 = left.parse().unwrap();
        let right_num: i32 = right.parse().unwrap();

        let result = if operator == '+' {
            left_num + right_num
        } else {
            left_num - right_num
        };

        *expr = result.to_string();
    }
}
