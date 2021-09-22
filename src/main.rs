use std::time::{SystemTime, UNIX_EPOCH};

mod record;

fn main() {
    println!("Hello! What is your mood right now?...");
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Did you not enter a string?");
    println!("Your mood right now is :{}", s);
    match record::record_mood(
        s,
        "mood.db",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
    ) {
        Ok(_) => println!("Well done!"),
        Err(_) => println!("Error!"),
    }
}
