fn main() {
    if let Err(e) = time_chooser::run() {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}
