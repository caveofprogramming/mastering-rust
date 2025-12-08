fn main() {
    /*
     * &&, ||, !, ==
     */

    let is_raining:bool = true;
    let is_cold:bool = false;

    println!("Raining: {is_raining}");

    let cold_and_raining = is_raining && is_cold;
    let take_coat = is_cold || is_raining;
    let nice_weather = !take_coat;
    
    let taking_coat = true;
    let wise_decision = take_coat == taking_coat;
    println!("Wise decision: {wise_decision}");

    println!("Cold and raining: {cold_and_raining}");
    println!("Take coat: {take_coat}");
    println!("Nice weather: {nice_weather}");

    println!("raining value: {}", is_raining as u8)
}
