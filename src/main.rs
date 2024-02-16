// Take input of password and filepath
// Read file contents and hash with given password
// Return hashed data out to new file

use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
#[arg(short, long)]
    path_to_file: std::path::PathBuf,
    #[arg(short, long)]
    key: String, 
}

fn write_file(path: &mut PathBuf, contents: &String) -> Result<()> {
    path.set_extension("enc");
    let mut file = File::create(path).expect("could not create file");
    write!(file, "{:?}", contents).expect("could not write to file");
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

    let _filevar = read_file(&mut args.path_to_file);
    let _fileout = write_file(&mut args.path_to_file, &_filevar);

    Ok(())
}

