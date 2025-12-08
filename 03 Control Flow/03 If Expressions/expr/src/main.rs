fn main() {
    let temperature = 25;

    let info = if temperature < 21 {
        "Take coat"
    }
    else {
        "It's warm"
    };

    println!("{info}");
}
