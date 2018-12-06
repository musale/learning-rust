#![allow(unused_variables)]
fn main() {
    // A string slice is a reference to part of a String

    let s = String::from("hello world");

    let w1 = &s[0..5];
    let w2 = &s[6..11];

    // The start..end syntax is a range that begins at
    // start and continues up to, but not including, end.
    // If I wanted to include end, we can use ..= i.e.
    // include the last index
    let w1 = &s[0..=4];
    let w2 = &s[6..=10];
    let w3 = first_word(&s);
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("word 1 {}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("word 2 {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word 3 {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
