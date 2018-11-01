fn main() {
    // SCALAR TYPES:
    // they represent a single value in a type
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("the sum is {}", sum);
    println!("the difference is {}", difference);
    println!("the product is {}", product);
    println!("the quotient is {}", quotient);
    println!("the remainder is {}", remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("the value of t is {} and for f is {}", t, f);

    // char is specified with a single quote unlike strings
    // which use double quotes
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("the c is {}, z is {} and the heart eyed cat is {}", c, z, heart_eyed_cat);

    // COMPOUND TYPE:
    // represent multiple values into one type i.e tuples and arrays

    // tuples have fixed length
    // written by writing comma-separated values inside parantheses 
    let tup: (i32, f64, u8) = (239, 6.3, 5);
    println!("the tuple here is {:?}", tup);

    // accessing a tuple value can be done using the dot ops on
    // the tuple variable followed by the index of the value

    let five = tup.2; // get the 3rd value in index 2
    println!("the third value in the tuple is {}", five);

    // you can also unpack
    let tup2 = (100, 0.5, "the III");
    let (hundred, point_five, the_third) = tup2;

    println!("the {} after {} is {}", hundred, point_five, the_third);

    // arrays are also of fixed length
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("the third month is {}", months[2]);
}
