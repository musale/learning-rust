fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // block conditions
    let next_number = if number % 3 == 0 {
        number * 3
    } else {
        number * 6
    };

    println!("the next number is {}", next_number);

    // using loop with a break condition
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // using while
    let mut num_three = 3;

    while num_three != 0 {
        println!("{}!", num_three);

        num_three = num_three - 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection using while
    let a = [10, 20, 30, 40, 50];
    let mut idx = 0;

    while idx < 5 {
        println!("the value is: {}", a[idx]);

        idx = idx + 1;
    }

    // looping through a collection using for
    for element in a.iter() {
        println!("the value with for is: {}", element);
    }

    // creating, reversing and looping through a list
    for number in (1..4).rev() {
        println!("rev value is {}!", number);
    }
    println!("LIFTOFF!!!");
}
