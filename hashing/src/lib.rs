use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    let count = list.len();
    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let len = sorted_list.len();
    if len == 0 {
        return 0;
    }

    if len % 2 == 0 {
        let mid1 = sorted_list[len / 2 - 1];
        let mid2 = sorted_list[len / 2];
        (mid1 + mid2) / 2
    } else {
        sorted_list[len / 2]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut frequency: HashMap<i32, i32> = HashMap::new();

    for &num in list {
        *frequency.entry(num).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;
    for (&num, &count) in &frequency {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }

    mode
}
