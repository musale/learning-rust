fn main() {
    let temp = 0.0;
    let celcius = converter(temp, false);
    let fahrenheit = converter(temp, true);
    println!(
        "converted {} degrees celcius to {} degrees fahrenheit",
        temp, fahrenheit
    );
    println!(
        "converted {} degrees fahrenheit to {} degrees celcius",
        temp, celcius
    );
}

// converts passed temp to celcius if true
// otherwise it's converted to fahrenheit
// (0°C × 9/5) + 32 = 32°F -> C to F
// (0°F − 32) × 5/9 = -17.78°C -> F to C
fn converter(temp: f64, celsius: bool) -> f64 {
    if celsius == true {
        (temp * 1.8) + 32.0 // return fahrenheit
    } else {
        (temp - 32.0) * 5.0 / 9.0 // return celcius
    }
}
