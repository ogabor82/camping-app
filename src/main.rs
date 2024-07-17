fn main() {
    let mut destination: &str  = "Crater Lake";
    println!("Destination: {}", destination);
    destination = "Sparks Lake";
    println!("Destination: {}", destination);

    const DESTINATION: &str = "Crater Lake";
    println!("Destination: {}", DESTINATION);
}
