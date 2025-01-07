pub fn delete_and_backspace(s: &mut String) {
    let mut index = 0;
    for i in s.chars() {
        if i == '+' {
            s.remove(index);
            index += 1;
            s.remove(index);
            break;

        }
        index += 1;
    }

    let mut indexneg = 0;
    for i in s.chars() {
        if i == '-' {
            s.remove(indexneg - 1);
            indexneg -= 1;
            s.remove(indexneg);
            break;
        }
        indexneg += 1;
    }

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
