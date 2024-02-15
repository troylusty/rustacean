// https://rust-cli.github.io/book/tutorial/

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    search: String,
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("search string: {:?}", &args.search);
    println!("file path: {:?}", &args.path);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.search) {
            println!("line: {}", line);
        }
    }

    Ok(())

}

