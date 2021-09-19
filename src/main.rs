mod record;

fn main() {
    println!("Hello! What is your mood right now?...");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Did you not enter a string?");
    println!("Your mood right now is :{}",s);
    record::record_mood();
}
