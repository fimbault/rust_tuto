fn main() {
    let s = String::from("item");

    // Add code here that calls the pluralize function

    println!(
        "I have one {}, you have two {}",
        s,
        pluralize(s.clone()),
    );
}

// Add appropriate parameters, return values, and implementation to this function
fn pluralize(singular: String) -> String {
    singular + "s"
}
