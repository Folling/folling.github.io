use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;

use anyhow::*;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let content = Path::new(&args[1]);

    if !content.exists() {
        panic!("Invalid path to content: {}", content.display());
    }

    println!("content path is {}", content.display());

    if !content.is_dir() {
        bail!("content path isn't a directory");
    }

    let posts = content.join("posts");
    let definitions = content.join("definitions");

    let subpaths = read_dir(posts)?;

    for path in subpaths {
        println!("{}", path?.path().display())
    }

    Ok(())
}
