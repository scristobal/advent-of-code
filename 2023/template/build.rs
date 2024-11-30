use reqwest::{blocking::Client, header::COOKIE};
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv()?;

    let current_dir = std::env::current_dir().unwrap();

    let dir_name = current_dir.file_name().unwrap().to_string_lossy();

    let Ok(day) = dir_name[4..].parse::<u8>() else {
        println!("directory name should be in the shape day-01");
        return Ok(());
    };

    let url = format!("https://adventofcode.com/2023/day/{}/input", &day);

    let session = std::env::var("SESSION").expect("SESSION env var should be set");

    let client = Client::new();
    let input_data = client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()?
        .text()?;

    let file_path = std::env::current_dir().unwrap().join("input.txt");
    let mut file = File::create(file_path).expect("should be able to create a file");

    file.write_all(input_data.as_bytes())
        .expect("should be able to write to input file");

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
