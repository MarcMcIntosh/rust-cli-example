use clap;
use clap::Parser;
use std::fs::DirEntry;
use std::io;
use std::path::{Path, PathBuf};
#[derive(Parser)]
#[command(name = "list_files")]
#[command(version = "1.0")]
#[command(about = "Lists Files", long_about = None)]
struct Cli {
    #[arg(default_value = ".")]
    path: String,
    #[arg(long, short, default_value_t = false)]
    recursive: bool,
}

type ListResult<T> = Result<Vec<T>, io::Error>;

fn list_path_recursive(path: PathBuf, depth: u32, dirs: Vec<PathBuf>) -> ListResult<PathBuf> {
    if dirs.contains(&path) {
        return Ok(dirs);
    } else if path.is_file() || depth == 0 {
        let result = [dirs, vec![path]].concat();
        return Ok(result);
    }

    let files: Vec<PathBuf> = path
        .read_dir()?
        .filter_map(Result::ok)
        .map(|dir_entry: DirEntry| list_path_recursive(dir_entry.path(), depth - 1, dirs.clone()))
        .collect::<ListResult<Vec<PathBuf>>>()?
        .into_iter()
        .flatten()
        .collect();

    return Ok(files);
}

fn list_path(path: PathBuf, recursive: bool) -> ListResult<PathBuf> {
    let dirs = Vec::<PathBuf>::new();
    let depth = if recursive { u32::MAX } else { 1 };
    return list_path_recursive(path, depth, dirs);
}

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path).to_path_buf();
    let list = list_path(path, cli.recursive);

    match list {
        Err(e) => eprintln!("Error: {:?}", e),
        Ok(files) => files.iter().for_each(|file| println!("{}", file.display())),
    }
}
