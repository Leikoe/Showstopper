use std::path::Path;
use clap::Parser;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CLI: Cli = Cli::parse();
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// minimum displayed file size (MB)
    #[arg(long, value_name = "SIZE", default_value = "2")]
    minsize: u64,
    
    /// maximum displayed file size (MB)
    #[arg(long, value_name = "SIZE")]
    maxsize: Option<u64>,
}

fn handle_path<P: AsRef<Path>>(path: P) {
    let p = path.as_ref();
    if p.is_dir() {
        handle_dir(p);
    } else if p.is_file() {
        handle_file(p);
    }
}

fn handle_dir(path: &Path) -> std::io::Result<()> {
    // get subdirs
    for entry in path.read_dir()? {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            handle_path(entry_path);
        }
    }
    
    Ok(())
}

fn handle_file(path: &Path) {
    let file_metadata = path.metadata().unwrap();
    let file_size = file_metadata.len() / 1e6 as u64;
    if file_size < CLI.minsize {
        return;
    }
    
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let path_name = path.display();
    println!("[{path_name}] - {file_name} - {file_size} MB");
}

fn main() -> std::io::Result<()> {
    handle_path(Path::new("/"));
    
    Ok(())
}
