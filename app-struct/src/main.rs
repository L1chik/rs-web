use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[0];

    if path.contains("/debug") {
        println!("The dev app is running");
    }
    else if path.contains("/release/") {
        print!("The prod is running");
    } else {
        panic!("unable ro run");
    }

    println!("{:?}", args)
}