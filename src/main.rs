use regex::Regex;
use sha1::{Digest, Sha1};
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

fn get_hash(filename: &str) -> io::Result<String> {
    let mut file = fs::File::open(filename)?;
    let mut sha1 = Sha1::new();

    let mut buffer = [0u8; 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        sha1.update(&buffer[..read]);
    }
    Ok(format!("{:x}", sha1.finalize()))
}

fn copy_if_needed(src_file: &str, dst_file: &str) -> io::Result<()> {
    if !Path::new(dst_file).exists() {
        fs::copy(src_file, dst_file)?;
        return Ok(());
    }
    let src_metadata = fs::metadata(src_file)?;
    let dst_metadata = fs::metadata(dst_file)?;
    if src_metadata.len() != dst_metadata.len()
        || src_metadata.modified()? != dst_metadata.modified()?
    {
        let src_hash = get_hash(src_file)?;
        let dst_hash = get_hash(dst_file)?;
        if src_hash != dst_hash {
            fs::copy(src_file, dst_file)?;
        }
    }
    Ok(())
}

fn copy_dir_tree(
    src_dir: &str,
    dst_dir: &str,
    ignore_patterns: &Vec<Regex>,
    total_files: &mut u64,
    copied_files: &mut u64,
) -> io::Result<()> {
    if !Path::new(dst_dir).exists() {
        fs::create_dir_all(dst_dir)?;
    }
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        if src_path.is_dir() {
            let item = src_path.file_name().unwrap().to_str().unwrap();
            let ignore = ignore_patterns.iter().any(|pattern| pattern.is_match(item));
            if !ignore {
                let dst_path = Path::new(dst_dir).join(item);
                copy_dir_tree(
                    src_path.to_str().unwrap(),
                    dst_path.to_str().unwrap(),
                    ignore_patterns,
                    total_files,
                    copied_files,
                )?;
            }
        } else {
            let item = src_path.file_name().unwrap().to_str().unwrap();
            let ignore = ignore_patterns.iter().any(|pattern| pattern.is_match(item));
            if !ignore {
                *total_files += 1;
                let dst_path = Path::new(dst_dir).join(item);
                copy_if_needed(src_path.to_str().unwrap(), dst_path.to_str().unwrap())?;
                *copied_files += 1;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let src_dir = "/Users/emirtuncer/Desktop/test";
    let dst_dir = "./test";
    let ignore_file = "ignore.txt";
    let mut ignore_patterns = Vec::new();

    for line in fs::read_to_string(ignore_file)?.lines() {
        ignore_patterns.push(Regex::new(line).unwrap());
    }

    // print ignore patterns
    for pattern in &ignore_patterns {
        println!("{}", pattern.as_str());
    }

    let mut total_files = 0;
    let mut copied_files = 0;
    copy_dir_tree(
        src_dir,
        dst_dir,
        &ignore_patterns,
        &mut total_files,
        &mut copied_files,
    )?;
    println!("Done. Copied {}/{} files.", copied_files, total_files);
    Ok(())
}
