#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
mod to_html;
use dirs;
use regex::Regex;

fn main() -> Result<(), io::Error> {
    let (src, out) = get_input()?;
    if src.extension().unwrap() != "md" {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"));
    }

    let mut content = fs::read_to_string(&src)?;
    let filename = src.file_name().unwrap().to_str().unwrap();
    let mut out_path = out.clone().join(filename);

    content = imports(&content, &PathBuf::from(src.parent().unwrap()));

    content = to_html::to_html(&content);
    content = content.replace("{{ TITLE }}", filename.split('.').next().unwrap());
    out_path.set_extension("html");
    fs::write(out_path, content)?;

    if let Some(mut assets) = dirs::home_dir() {
        assets.push("Documents");
        assets.push("Presentazioni");
        assets.push("output");

        copy_files(&assets, &out)?;
    } else {
        println!("Unable to determine home directory");
    }
    Ok(())
}

fn get_input() -> Result<(PathBuf, PathBuf), io::Error> {
    let mut src = "./source/beta.md";
    let mut out = "./output2";

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
pub fn files_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let walker = fs::read_dir(source_folder)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| entry.path().canonicalize().ok())
        .collect::<Vec<PathBuf>>();
    Ok(walker)
}

fn imports(content: &str, src: &PathBuf) -> String {
    let re = Regex::new(r#"\[\[(.*?)\]\]"#).unwrap();
    let mut out = content.to_string();

    for cap in re.captures_iter(content) {
        let path = src.join(&cap[1]);
        let imported = match fs::read_to_string(&path) {
            Ok(mut content) => {
                content = to_html::md_content_into_html(&content);
                "<section class='comment'>".to_string() + &content + "</section>\n"
            }

            Err(_) => format!("[[{}]]", &cap[1]),
        };
        out = out.replace(&cap[0], &imported);
    }
    out
}

// Copy the files and folders from the source folder to the destination folder
pub fn copy_files(src: &PathBuf, out: &PathBuf) -> io::Result<()> {
    dirs_walker(src)?
        .iter()
        .chain(std::iter::once(src))
        .map(|path| {
            let dest = out.join(&path.strip_prefix(src).unwrap());
            if !dest.exists() {
                fs::create_dir_all(&dest).unwrap();
            }
            path
        })
        .filter_map(|dirs| files_walker(dirs).ok())
        .flatten()
        .map(|file| {
            let path = file.strip_prefix(src).unwrap().to_path_buf();
            (file, out.join(path))
        })
        .filter_map(|(src, out)| fs::copy(src, out).ok())
        .for_each(|_| ());

    Ok(())
}

// return the list of directories contained in the source folder
// and recursively in its subfolders
pub fn dirs_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut walker = fs::read_dir(source_folder)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| entry.is_dir())
        .filter_map(|dir| dir.canonicalize().ok())
        .collect::<Vec<PathBuf>>();

    // recursively add subfolders
    walker.extend(
        walker
            .clone()
            .iter()
            .filter_map(|path| dirs_walker(&path).ok())
            .flatten(),
    );
    Ok(walker)
}
