
mod temperature;

fn main() {
    let cel_temp = 36.0;
    let fah = temperature::transfer(cel_temp, temperature::TemperatureType::Fahrenheit);
    let cel = temperature::transfer(fah, temperature::TemperatureType::Celsius);

    println!("the temperature is {fah} Celsius(華氏)");
    println!("the temperature is {cel} Celsius(攝氏)");
}
