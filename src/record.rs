use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs::OpenOptions, io::Write};

pub fn record_mood(s: String, output_file: &str, t: Duration) -> std::io::Result<()> {
    //create the file
    match File::open(output_file) {
        Ok(_) => println!("mood.db* exists"),
        Err(_) => match File::create(output_file) {
            Ok(_) => println!("mood.db* file does not exist... creating file"),
            Err(n) => println!("Failed to create file {}", n),
        },
    }

    //append to the file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file)?;
    let r = format!("{:?},{}", t, s);
    file.write(r.as_bytes())?;
    Ok(())
}

#[test]
fn test_record_mood() -> std::io::Result<()> {
    let time_stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mood = "Happy";

    File::create("mood.test.db")?;

    match record_mood(String::from(mood), "mood.test.db", time_stamp) {
        Ok(_) => println!("Test file generated"),
        Err(a) => println!("Failed to generate test file: {}", a),
    }

    //verify test file contents
    let test_file = File::open("mood.test.db")?;
    let mut buf_reader = BufReader::new(test_file);
    let mut test_file_contents = String::new();
    buf_reader.read_to_string(&mut test_file_contents)?;
    assert_eq!(test_file_contents, format!("{:?},{}", time_stamp, "Happy"));

    fs::remove_file("mood.test.db")?;
    Ok(())
}
