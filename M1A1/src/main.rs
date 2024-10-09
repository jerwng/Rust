const WATER_FREEZING_POINT_FARENHEIT:f64 = 32.0;

fn main() {
    let mut temp_f = 69.0;

    println!("{}°F in Celsius is {}°C", temp_f, farenheit_to_celsius(temp_f));

    let mut counter = 0;

    loop {
        counter += 1;
        temp_f += 1.0;

        if counter > 5 {
            break;
        }

        println!("{}°F in Celsius is {}°C", temp_f, farenheit_to_celsius(temp_f) as i64);
    }
}

fn farenheit_to_celsius(f: f64) -> f64 {
    return (f - WATER_FREEZING_POINT_FARENHEIT) / (9.0/5.0);
}

fn celsius_to_farenheit(c: f64) -> f64 {
    return (c * 9.0/5.0) + WATER_FREEZING_POINT_FARENHEIT;
}