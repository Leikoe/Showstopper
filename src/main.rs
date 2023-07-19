use std::{path::Path, fs::metadata};


const MIN_FILE_SIZE_MB: u64 = 500;

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
    if file_size < MIN_FILE_SIZE_MB {
        return;
    }
    
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let path_name = path.display();
    println!("[{path_name}] - {file_name} - {file_size} MB");
}

fn main() -> std::io::Result<()> {
    // get fs root
    let path = Path::new("/");
    handle_path(path);
    
    Ok(())
}
