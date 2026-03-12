// use std::fs;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
#[command(name = "list_files")]
#[command(version = "1.0")]
#[command(about = "Lists Files", long_about = None)]
struct Cli {
    #[arg(default_value=".")]
    path: String,
    #[arg(long, short, default_value_t=false)]
    recursive: bool,
}


fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);
    println!("path: {:?}", path);
    
}

