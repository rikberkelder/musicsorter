use std::env;
use std::ffi::OsStr;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use walkdir;

const MUSIC_FILE_EXTENSIONS: [&'static str; 2] = ["mp3", "flac"];

#[derive(Debug)]
struct Album {
    path: PathBuf,
}

impl Album {
    fn new(path: PathBuf) -> Album {
        Album {
            path: PathBuf::from(path),
        }
    }
}

fn get_args() -> Vec<String> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    return arguments;
}

fn get_directory_contents(path: &Path) -> Vec<walkdir::DirEntry> {
    let mut contents: Vec<walkdir::DirEntry> = Vec::new();
    for entry in walkdir::WalkDir::new(path).min_depth(1).max_depth(1) {
        let entry: walkdir::DirEntry = entry.unwrap();
        contents.push(entry);
    }
    return contents;
}

fn get_extension_from_path(path: &Path) -> Option<&str> {
    return path.extension().and_then(OsStr::to_str);
}

fn has_music(path: &Path) -> bool {
    let mut has_music: bool = false;
    let directory: Vec<walkdir::DirEntry> = get_directory_contents(&path);
    for entry in directory {
        if let Some(extension) = get_extension_from_path(entry.path()) {
            if MUSIC_FILE_EXTENSIONS.contains(&extension) {
                has_music = true;
                break;
            }
        }
    }
    return has_music;
}

fn get_albums_recursively(path: &Path, mut albums: &mut Vec<Album>) {
    let is_album: bool = has_music(path);
    if is_album {
        albums.push(Album::new(path.to_path_buf()));
    } else {
        let subdir: Vec<walkdir::DirEntry> = get_directory_contents(path);
        for entry in subdir {
            let metadata: Metadata = entry.metadata().unwrap();
            if metadata.is_dir() {
                get_albums_recursively(&entry.path(), &mut albums);
            }
        }
    }
}

fn main() {
    let mut albums: Vec<Album> = Vec::new();
    for argument in get_args() {
        let path: &Path = &Path::new(&argument);
        get_albums_recursively(&path, &mut albums);
    }
    println!("{:#?}, {}", albums, albums.len());
}
