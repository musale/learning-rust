#![allow(unused_variables)]
fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter(UsState::Alabama);
    let coin5 = Coin::Quarter(UsState::Alaska);

    value_in_cents(coin1);
    value_in_cents(coin2);
    value_in_cents(coin3);
    value_in_cents(coin4);
    value_in_cents(coin5);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
