// Take input of password and filepath
// Read file contents and hash with given password
// Return hashed data out to new file

use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    key: String,
    #[arg(short, long)]
    file_path: std::path::PathBuf,
}

fn write_file(file_path: &mut PathBuf, contents: String) -> std::io::Result<()> {
    file_path.set_extension("enc");
    let mut file = File::create(file_path)?;
    write!(file, "{}", contents).expect("could not write to file");
    Ok(())
}

fn read_file(file_path: &mut PathBuf) -> String {
    let file = File::open(file_path).expect("no such file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    contents
}

fn main() -> Result<()> {
    let mut args = Cli::parse();

    println!("key: {:?}, file path: {:?}", &args.key, &args.file_path);

    let _filevar = read_file(&mut args.file_path);
    let _fileout = write_file(&mut args.file_path, _filevar);

    let content = std::fs::read_to_string(&args.file_path)
        .with_context(|| format!("could not read file `{}`", args.file_path.display()))?;

    for line in content.lines() {
        if line.contains(&args.key) {
            println!("line: {}", line);
        }
    }

    Ok(())
}

