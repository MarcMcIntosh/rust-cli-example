use clap;
use clap::Parser;
use std::path::Path;
use list_files::ls;
#[derive(Parser)]
#[command(name = "list_files")]
#[command(version = "1.0")]
#[command(about = "Lists Files", long_about = None)]
struct Cli {
    #[arg(default_value = ".")]
    path: String,
    #[arg(long, short, default_value_t = false)]
    recursive: bool,
    #[arg[long, short, default_value_t = false]]
    all: bool,
}

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path).to_path_buf();

    ls(path, cli.recursive, cli.all).iter().for_each(|f| {
        println!("{:?}", f.as_os_str());
    });
}
