use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir;

const music_file_extensions: [&'static str; 2] = ["mp3", "flac"];

fn get_args() -> Vec<String> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let mut args: Vec<String> = Vec::new();
    for argument in arguments {
        args.push(argument);
    }
    return args;
}

fn get_directory_contents(path: &Path) -> Vec<walkdir::DirEntry> {
    let mut contents: Vec<walkdir::DirEntry> = Vec::new();
    for entry in walkdir::WalkDir::new(path).max_depth(1) {
        contents.push(entry.unwrap());
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
            println!("{:#?} has extension {:#?}", &path, extension);
            if (music_file_extensions.contains(&extension)) {
                has_music = true;
            }
        }
    }
    return has_music;
}

fn main() {
    for argument in get_args() {
        println!("{:#?}, {:#?}", &argument, has_music(Path::new(&argument)));
    }
}
