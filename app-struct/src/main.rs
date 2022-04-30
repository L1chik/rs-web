use std::collections::HashMap;

fn main() {
    let mut gmap = HashMap::new();
    gmap.insert("test", 25);

    match gmap.get("test") {
        None => eprintln!("There is no such key"),
        Some(t) => println!("Result is: {}", t)
    }
}
