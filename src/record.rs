use std::{fs::{File}, io::Write};

pub fn record_mood() -> std::io::Result<()>{
    println!("Mood recorded");
    let mut file = File::create("mood.db")?;
    file.write_all(b"Hello, Esteban!")?;
    Ok(())
}