extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate walkdir;

use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use walkdir::WalkDir;

const CRATE_SRC: &'static str = "registry+https://github.com/rust-lang/crates.io-index";

#[derive(Deserialize, Debug)]
struct Package<'a> {
    name: &'a str,
    version: &'a str,
    source: Option<&'a str>,
}

fn main() -> Result<(), Box<Error>> {
    let mut args = env::args();
    args.next().ok_or("cargo")?;
    args.next().ok_or("cleanup")?;
    let file = args.next().unwrap_or(String::from("Cargo.lock"));
    println!("{}", file);
    let file = File::open(file)?;
    let file = BufReader::new(file);

    let mut package = String::new();
    let mut version_map = HashMap::new();
    for line in file.lines() {
        let line = line?;
        if &line == "[[package]]" {
            package.clear();
            continue;
        }
        if &line == "" {
            let one_package: Package = toml::from_str(&package)?;
            match one_package.source {
                None => continue,
                Some(s) if s != CRATE_SRC => continue,
                _ => (),
            }
            version_map.entry(one_package.name.to_owned()).or_insert(HashSet::new()).insert(one_package.version.to_owned());
        }
        package.push_str(&line);
        package.push('\n');
    }

    let pkg_dirs = WalkDir::new(unpacked_crate_path()?).min_depth(1).max_depth(1).into_iter();
    for entry in pkg_dirs.filter_entry(|e| e.file_type().is_dir()) {
        if let Some(path) = entry?.path().file_name().map(OsStr::to_str).unwrap_or(None) {
            let mut pkg_name_vers = path.split('-');
            if let Some(version) = pkg_name_vers.next_back() {
                let name = &path[..path.len() - version.len() - 1];
                if let Some(versions) = version_map.get(name) {
                    if !versions.contains(version) {
                        println!("{}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

fn unpacked_crate_path() -> Result<PathBuf, Box<Error>> {
    let mut src = PathBuf::new();
    src.push(env::var("HOME")?);
    src.push(".cargo/registry");
    let mut cache = src.clone();
    cache.push("cache");
    let cache = WalkDir::new(cache).min_depth(1).max_depth(1).into_iter();
    for entry in cache.filter_entry(|e| e.file_type().is_dir()) {
        src.push("src");
        src.push(entry?.path().file_name().map(OsStr::to_str).unwrap_or(None).ok_or("registry/cache")?);
        break;
    }
    Ok(src)
}
