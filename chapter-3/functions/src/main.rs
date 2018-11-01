fn main() {
    print_integer(23, 45);
    block_example(30);
    let res: i32 = return_example(10);
    println!("the returned value is {}", res);
}


fn print_integer(x: i32, y:u32){
    println!("the signed integer is {} and unsigned integer is {}", x, y);
}


// Expressions evaluate to something i.e.
// - summing 5+6 evaluates to 11
// - calling a function, macro is an expression
// - the block {} used to create new scopes is also an expression
// - they DO NOT include ending semicolons as including them turns them
// into statements which don't return values

fn block_example(x: i32) {
    let y: i32 = 20;
    let z = {
        let j = 10;
        j + y + x
    };
    println!("block sum is {}", z);
}

// functions which return a value can use the return keyword
// explicitly or have the expression being evaluated not to end
// with semicolon

fn return_example(x: i32) -> i32{
    x * 10
}
