pub fn run() {
    // print to console
    println!("Hello from print rs file");

    // basic formating
    println!("Number: {}", 1);

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Iulian", "TM", "code"
    );

    // named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Soccer"
    );

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug traits
    println!("{:?}", (12, true, "Hi"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
