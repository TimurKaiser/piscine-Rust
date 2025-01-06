pub fn fibonacci(n: u32) -> u32 {
    let mut x = 0;
    let mut y = 1;

    // if n < 0 {                       // u32 exclude negative number we don't need that
    //
    // }
    if n == 0 {        
        return x;
    }
    if n == 1 {
        return y;
    }

    let mut i = 2;
    
    while i <= n {
        let temp = x + y;
        x = y;
        y = temp;
        i +=1;
    }

    return y 

}