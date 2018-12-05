fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // the ampersand allows us to make a reference to a value
    // without taking ownership

    println!("The length of '{}' is {}.", s1, len);

    // A data race is similar to a race condition and happens when
    // these three behaviors occur:

    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.

    let mut s2 = String::from("hello");

    {
        let r1 = &mut s2;
        println!("{}", r1)
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s2;
    println!("{}", r2)

    // Rules of referencing
    // - At any given time, you can have either (but not both of)
    // one mutable reference or any number of immutable references.
    // - References must always be valid.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
