use std::default;
use std::env;
use std::fs;
use std::fs::DirEntry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut env_dir: &str = "";

    if args.len() == 2 && !args[2].contains("-") {
        env_dir = args[2].as_str();
    } else if args.len() == 1 {
        env_dir = "./";
    }

    let flag_str: &str;
    let mut env_dir = args[args.len() - 1].as_str();
    let mut flag_lst: Option<Vec<&str>> = Some(vec![]);

    if args.len() == 2 {
        let flags = args[1].as_str();
        if flags.chars().nth(0).unwrap() == '-' {
            flag_str = &flags[1..];
            flag_lst = Some(flag_str.split("").collect());
            env_dir = args[2].as_str();
        }
        panic!("Unsupported arguments");
    } else if args.len() == 3 {
        let flags = args[1].as_str();
        if flags.chars().nth(0).unwrap() == '-' {
            flag_str = &flags[1..];
            env_dir = &"";
        } else {
            flag_lst = None;
            env_dir = ""
        }
    } else if args.len() > 2 {
        panic!("Too many args");
    }

    // for flag in flag_lst.unwrap() {
    //     match flag {
    //         &_ => {}
    //     }
    // }

    let paths = fs::read_dir("./").unwrap();
    let print = |path: DirEntry| -> Option<String> {
        let mut out = String::from("");
        let path_str = path.path();
        if path_str.is_dir() {
            out = String::from("|>");
        }
        let res = path_str.file_name()?.to_str()?;
        out.push_str(res);
        return Some(out);
    };
    for path in paths {
        print!("{}\n", print(path.unwrap()).unwrap());
    }
}
