use std::path::{Path, PathBuf};

use mp3_duration;
use rayon::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let path = Path::new(&args.path);
    let entries: Vec<PathBuf> = path
        .read_dir()
        .expect("path needs to exist")
        .map(|entry| entry.unwrap().path())
        .collect();
    let total: u64 = entries
        .par_iter()
        .map(|path| {
            let ext = path.extension();
            if ext.is_some() && ext.unwrap() == "mp3" {
                let duration = mp3_duration::from_path(&path).unwrap();
                duration.as_secs() * 1000 + duration.subsec_millis() as u64
            } else {
                0
            }
        })
        .sum();
    println!("{}", total);
}
