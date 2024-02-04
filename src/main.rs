#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
mod to_html;
use regex::Regex;

fn main() -> Result<(), io::Error> {
    let (src, out) = get_input()?;
    for path in files_walker(&src)? {
        if path.extension().unwrap() != "md" {
            continue;
        }

        let mut content = fs::read_to_string(&path)?;
        let filename = path.file_name().unwrap().to_str().unwrap();
        let mut out_path = out.clone().join(filename);

        content = imports(&content, &src);
        content = to_html::to_html(&mut content);
        content = content.replace("{{ TITLE }}", filename.split('.').next().unwrap());
        out_path.set_extension("html");
        fs::write(out_path, content)?;
    }

    Ok(())
}

fn get_input() -> Result<(PathBuf, PathBuf), io::Error> {
    let mut src = "./source";
    let mut out = "./output";

    let args = env::args().collect::<Vec<_>>();
    let mut args_iter = args[1..].iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-o" => out = args_iter.next().unwrap(),
            "-s" => src = args_iter.next().unwrap(),
            _ => {
                // help
                println!("Usage: {} [-d destination] [-t target]", args[0]);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"));
            }
        }
    }

    if !PathBuf::from(out).exists() {
        std::fs::create_dir_all(out)?;
    }

    Ok((
        PathBuf::from(src).canonicalize().unwrap(),
        PathBuf::from(out).canonicalize().unwrap(),
    ))
}

// return the list of files contained in the source folder
// and recursively its subfolders
pub fn files_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let walker = fs::read_dir(source_folder)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| entry.path().canonicalize().ok())
        .collect::<Vec<PathBuf>>();
    Ok(walker)
}

fn imports(content: &str, src: &PathBuf) -> String {
    let re = Regex::new(r#"\\import\[\[(.*?)\]\]"#).unwrap();
    let mut out = content.to_string();

    for cap in re.captures_iter(content) {
        let path = src.join(&cap[1]);
        let imported = fs::read_to_string(&path).unwrap();
        out = out.replace(&cap[0], &imported);
    }
    out
}
