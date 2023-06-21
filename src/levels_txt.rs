use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;

const EMPTY_LEVEL: &str = "empty\r\n32,18,00,00,L\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n................................\r\n00\r\n000000\r\n\r\n";

pub fn init_levels() -> std::io::Result<()>
{
    // Create a levels.txt if it doesn't already exist
    if File::open("levels.txt").is_err() 
    {
        println!("levels.txt not found, creating...");
        File::create("levels.txt")?;
    }

    let mut levels_txt = OpenOptions::new().write(true).open("levels.txt")?;
    levels_txt.set_len(0)?;

    let mut template = String::new();
    template.push_str("loadedLevels=\r\n");

    for _ in 1..=53
    {
        template.push_str(EMPTY_LEVEL);
    }

    levels_txt.write_all(template.as_bytes()).unwrap();
    Ok(())
}

pub fn add_level(_level_string: String) -> Result<(), Box<dyn Error>>
{
    unimplemented!();
}