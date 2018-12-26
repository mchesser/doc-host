use std::error::Error;
use std::fs;
use std::path::Path;

use fs_extra;

use mdbook::MDBook;

use config::Config;

pub fn update_book(config: &Config) -> Result<(), Box<Error>> {
    build_book(&config.tmp_dir.join("src"))?;

    let src_path = config.tmp_dir.join("src").join("book");
    let dst_path = config.tmp_dir.join("dst").join("book");

    // Remove old version
    if dst_path.exists() {
        println!("Removing old book version");
        fs::remove_dir_all(&dst_path)?;
    }

    println!("Copying from {:?}, to {:?}", src_path, dst_path);
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;

    fs_extra::dir::copy(&src_path, &dst_path, &options)?;

    Ok(())
}

fn build_book<P: AsRef<Path>>(book_root_dir: P) -> Result<(), Box<Error>> {
    println!("Building book");
    let book = MDBook::load(book_root_dir.as_ref())?;
    book.build()?;

    Ok(())
}

