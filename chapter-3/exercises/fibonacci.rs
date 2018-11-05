fn main() {
    let nth = 9;
    let fibonacci_number = get_nth_fibonacci(nth);
    println!("the {} fibonacci number is {}", nth, fibonacci_number);
}

// returns the nth fibonacci number recursively
// Fn = Fn-1 + Fn-2
// F0 = 0 and F1 = 1.
fn get_nth_fibonacci(nth: i32) -> i32 {
    if nth == 0 {
        0
    } else if nth == 1 {
        1
    } else {
        get_nth_fibonacci(nth - 1) + get_nth_fibonacci(nth - 2)
    }
}
