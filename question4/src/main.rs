use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader, Write};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.ends_with(".txt") {
            let path = Path::new(entry.path());
            let file = File::open(&path)?;
            let file = BufReader::new(file);
            let mut cnt = 0;
            for _ in file.lines() {
                cnt += 1;
            }
            io::stdout().write_fmt(format_args!("{}:  {} lines\n", file_name, cnt))?;
        }
    }

    Ok(())
}
