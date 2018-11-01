fn main() {
    // let x = 10; <- this is an immutable assign which will fail
    // if we re-assign x
    // To avoid errors, make x mutable like so;
    let mut x = 10;
    println!("the value of x is {}", x);
    x = x + 5;
    println!("the new value of x after adding 5 is {}", x);

    // constant MUST not have mut keyword as it's immutable
    // it MUST have assigned type
    const MAX_POINTS: u32 = 100_000;

    println!("the maximum points are {}", MAX_POINTS);

    // shadowing is reusing a declared variable as a new
    // totally different variable

    let spaces = "   ";
    let spaces = spaces.len();

    println!("the length of the spaces is {}", spaces);
}
