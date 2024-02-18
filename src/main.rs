use anyhow::Result;
use clap::Parser;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    path_to_file: std::path::PathBuf,
    #[arg(short, long)]
    key: String,
}

fn read_file(file_path: &PathBuf) -> String {
    let mut contents = Default::default();
    if file_path.extension().and_then(OsStr::to_str) == Some("txt") {
        let file = File::open(file_path).expect("no such file");
        let mut buf_reader = BufReader::new(file);
        contents = String::new();
        let _ = buf_reader.read_to_string(&mut contents);
    }
    contents = hash_data(&contents);
    contents
}

fn hash_data(contents: &String) -> String {
    let mut append_string: String = Default::default();
    for c in contents.chars() {
        println!("{}", c);
        // 'Encrypt' each char or whitespaced word then 
        // push_str each looped element back together.
    }
    append_string
}

fn write_file(path: &mut PathBuf, contents: &String) -> Result<()> {
    path.set_extension("enc");
    let mut file = File::create(path).expect("could not create file");
    write!(file, "{}", contents).expect("could not write to file");
    Ok(())
}

fn main() -> Result<()> {
    let mut args = Cli::parse();
    println!("File: {:?}, Key: {:?}", &args.path_to_file, &args.key);
    let _filevar = read_file(&args.path_to_file);
    let _fileout = write_file(&mut args.path_to_file, &_filevar);
    Ok(())
}
