use std::env;
use std::fs;
use std::fs::DirEntry;
use std::vec;

fn main() {
    // let args: Vec<String> = env::args().collect();
    //
    //if a path is specified use it, if not then set path to "./"
    // set depth to 0 by default
    // -s should hide . files
    // add more tags for more info
    let mut depth = 0;
    let mut hide = false;
    let mut path = String::from("./");

    let mut args: Vec<String> = env::args().collect();
    let mut has_path: bool = false;
    let mut path_ind: usize = 0;
    args.remove(0);
    let mut flags: Vec<char> = vec![];

    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with("-") {
            let flag_spread = &arg[1..];
            if flag_spread.starts_with("-d=") {
                depth = flag_spread[3..4].parse::<i32>().unwrap();
                continue;
            }

            for el in flag_spread.chars().enumerate() {
                flags.push(el.1);
            }
        } else {
            has_path = true;
            path_ind = i;
            return;
        }
        has_path = false;
    }

    for flag in flags {
        match flag {
            's' => hide = true,
            x => error(format!("Unexpected flag: {}", x)),
        }
    }

    if has_path {
        path.push_str(args.get(path_ind).unwrap());
    }

    read_dir(path, depth + 1, 0, hide);
}

fn error(msg: String) {
    print!("Error: {}\n", msg);
}

fn read_dir(path: String, depth: i32, ind: i32, hide: bool) {
    if depth == ind {
        return;
    }
    let mut dirs: Vec<DirEntry> = vec![];
    let mut files: Vec<DirEntry> = vec![];
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        if let Ok(path) = path {
            if path.path().is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }

    dirs.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    print_files(dirs, depth, ind, hide);
    print_files(files, depth, ind, hide);
}

fn print_files(files: Vec<DirEntry>, depth: i32, ind: i32, hide: bool) {
    for file in files {
        print_file(file, depth, ind, hide);
    }
}

fn print_file(file: DirEntry, depth: i32, ind: i32, hide: bool) -> Option<bool> {
    if ind > depth {
        return Some(true);
    }
    if let Some(path) = file.path().file_name() {
        if !hide || !path.to_str()?.starts_with(".") {
            if file.path().is_dir() {
                print_depth(ind);
                print!("{}|>\n", path.to_str()?);
                if ind < depth {
                    read_dir(
                        String::from(file.path().to_str().unwrap()),
                        depth,
                        ind + 1,
                        hide,
                    );
                }
            } else {
                print_depth(ind);
                print!("{}~\n", path.to_str()?)
            }
        }
    }
    return Some(true);
}

fn print_depth(depth: i32) {
    if depth == 0 {
        return;
    }
    print!("\t");
    print_depth(depth - 1);
}
