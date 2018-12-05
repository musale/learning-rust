fn main() {
    // Owner ship rules
    // ================
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // DEMO: Each value has a variable that's called it's owner
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
                         // do stuff with s
        println!("the value of s is {}", s);
    } // this scope is now over, and s is no longer valid

    {
        let s1 = String::from("hello"); // s1 is allocated memory in the stack
        let s2 = s1; // s1 is moved to s2
                     // When we assign s1 to s2, the String data is copied, meaning
                     // we copy the pointer, the length, and the capacity that are on
                     // the stack. We do not copy the data on the heap that the pointer refers to.
        println!("the value of s2 is {}", s2);
    } // the scope is over and s2 is no longer valid

    // any group of simple scalar values can be Copy, and nothing
    // that requires allocation or is some form of resource is Copy.
    // Here are some of the types that are Copy:

    // - All the integer types, such as u32.
    // - The Boolean type, bool, with values true and false.
    // - All the floating point types, such as f64.
    // - The character type, char.
    // - Tuples, but only if they contain types that are also Copy.
    // For example, (i32, i32) is Copy, but (i32, String) is not.

    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it’s okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("some string is {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("some integer is {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
