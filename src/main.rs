use std::fs::File;
use std::time::{Duration, Instant};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("logs.txt")?;
    file.write_all(b"This works?")?;
    Ok(())
}
