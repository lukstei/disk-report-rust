use std::{fs};
use std::path::{PathBuf};
use itertools::Itertools;

struct Tree {
    path: PathBuf,
    total_size: u64,
    own_size: u64,
    children: Vec<Tree>,
}

#[derive(Debug)]
enum Error{
    IO(std::io::Error)
}

fn read_tree(path: PathBuf, file_count: &mut u64) -> Result<Tree, Error> {
    let mut children = Vec::new();

    let mut own_size = 0;
    let mut total_size = 0;
    for e in fs::read_dir(&path).map_err(Error::IO)? {
        *file_count += 1;

        if *file_count % 100000 == 0 {
            eprintln!("Read {} files, last: {}", file_count, path.to_str().unwrap_or("??"));
        }

        let e = e.map_err(Error::IO)?;
        let metadata = e.metadata().map_err(Error::IO)?;
        if metadata.is_dir() {
            match read_tree(e.path(), file_count) {
                Ok(result) => {
                    total_size += result.total_size;
                    children.push(result);
                }
                Err(err) => {
                    eprintln!("Error reading {}: {:?}", e.path().to_str().unwrap_or("??"), err);
                }
            };
        } else if metadata.is_file() {
            own_size += metadata.len();
            total_size += metadata.len();
        }
    }

    children.sort_by_key(|x| x.total_size);
    children.reverse();

    Ok(Tree {
        path,
        total_size,
        own_size,
        children,
    })
}

fn format_tree(t: &Tree, depth: i32, total_size: u64) -> String {
    let cutoff = 0.01;

    let f = t.total_size as f64 / total_size as f64;
    let children = t.children.iter()
        .filter(|t| t.total_size as f64 / total_size as f64 > cutoff)
        .map(|t| format_tree(t, depth + 1, total_size)).join("\n");

    format!("{}{} - {:.0}%, {:.0}MB total / {:.0}MB own{}",
            " ".repeat((depth * 2) as usize),
            t.path.file_name().unwrap_or_else(||t.path.as_os_str()).to_str().unwrap_or("??"),
            f * 100.,
            t.total_size as f64 / 1e6,
            t.own_size as f64 / 1e6,
            if children.is_empty() { "".to_string() } else { format!("\n{}", children) }
    )
}

use clap::Parser;

/// An application for displaying the relative size of the file contents of a directory tree
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory to scan
    #[clap(default_value = ".")]
    directory: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut file_count = 0;
    let path = fs::canonicalize(&args.directory).map_err(Error::IO)?;

    println!("Disk report for {}\n", path.to_str().unwrap_or("??"));
    let tree = read_tree(path, &mut file_count)?;
    println!("{}", format_tree(&tree, 0, tree.total_size));
    Ok(())
}
